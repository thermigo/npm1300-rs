# Rust nPM1300 PMIC Driver

A `no_std`, async Rust driver for the [Nordic nPM1300](https://www.nordicsemi.com/Products/nPM1300) Power Management IC (PMIC).  
This crate provides both low-level register access and a high-level API for managing PMIC functions.

## Features

- `no_std` support for embedded environments
- Async design
- Type-safe register access
- [`defmt`](https://github.com/knurling-rs/defmt) support for logging (optional)
- Generated low-level API using [`device-driver`](https://docs.rs/device-driver/)
- Minimal dependencies

## API Coverage

> [!IMPORTANT]
> The driver is currently under development, and the API may change before reaching 1.0.

| **Component**                             | **Low-Level API** | **High-Level API** |
| ----------------------------------------- | :---------------: | :----------------: |
| SYSREG — System regulator                 |        ✅         |         ✅         |
| CHARGER — Battery charger                 |        ✅         |         ✅         |
| BUCK — Buck regulators                    |        ✅         |         ✅         |
| LOADSW — Load switches                    |        ✅         |         ✅         |
| LDO — LDO regulators                      |        ❌         |         ❌         |
| LEDDRV — LED drivers                      |        ✅         |         ✅         |
| GPIO — General-purpose I/O                |        ✅         |         ✅         |
| ADC - System Monitor                      |        ✅         |         ⚠️         |
| POF - Power-fail comparator               |        ✅         |         ✅         |
| TIMER — Timer/monitor                     |        ❌         |         ❌         |
| Ship and hibernate modes                  |        ✅         |         ✅         |
| Event and interrupt                       |        ❌         |         ❌         |
| Reset and error                           |        ❌         |         ❌         |
| Fuel gauge                                |        ❌         |         ❌         |

Legend:

- ✅ Fully implemented (at least should be)
- ⚠️ Implemented but has known issues (see [Issues](https://github.com/thermigo/npm1300-rs/issues))
- ❌ Not yet implemented

> [!WARNING]
> While core functionality has been tested, this driver is not yet production-ready. Contributions and bug reports are welcome!

> [!NOTE]
> This crate is async-only and there are no plans to add synchronous APIs. Contributions are welcome!

## Usage Example

Here's a minimal example using the Embassy framework on an nRF52840:

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    peripherals,
    twim::{self, Twim},
};
use {defmt_rtt as _, panic_probe as _};

use npm1300_rs::{
    types::BuckVoltage,
    NPM1300,
};

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let config = twim::Config::default();

    let twi = Twim::new(p.TWISPI0, Irqs, p.P0_07, p.P0_12, config);

    let mut npm1300 = NPM1300::new(twi);
    let _ = npm1300.set_buck2_normal_voltage(BuckVoltage::V1_8).await;
    let _ = npm1300.enable_buck2().await;
}
```

More examples can be found in the [`examples`](examples) directory.

## Support

- [GitHub Issues](https://github.com/user/npm1300-rs/issues) - Bug reports, feature requests, and questions

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch
3. Follow [conventional commits](https://www.conventionalcommits.org) for commit messages
4. Submit a Pull Request

By contributing, you agree that your work will be dual-licensed as above, without additional terms or conditions.
