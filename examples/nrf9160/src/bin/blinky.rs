#![no_std]
#![no_main]

//! Example demonstrating control of the NPM1300 PMIC's leds

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    peripherals,
    twim::{self, Twim},
};
use embassy_time::Timer;

use {defmt_rtt as _, panic_probe as _};

use npm1300_rs::{
    leds::LedMode,
    NPM1300,
};

bind_interrupts!(struct Irqs {
    SERIAL0 => twim::InterruptHandler<peripherals::SERIAL0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    
    let sdapin = p.P0_28;
    let sclpin = p.P0_29;
    let mut config = twim::Config::default();

    // Modify the i2c configuration fields if you dont have external i2c pullups
    config.sda_pullup = true;
    config.scl_pullup = true;

    defmt::info!("Configuring TWIM...");
    let twi = Twim::new(p.SERIAL0, Irqs, sdapin, sclpin, config);
    
    let mut npm1300 = NPM1300::new(twi, embassy_time::Delay);
    defmt::info!("Configuring LED modes...");
    let _ = npm1300.configure_led0_mode(LedMode::Host).await;
    defmt::info!("Configured LED0 mode to Host");
    let _ = npm1300.configure_led1_mode(LedMode::Charging).await;
    defmt::info!("Configured LED1 mode to Charging");
    let _ = npm1300.configure_led2_mode(LedMode::ChargingError).await;
    defmt::info!("Configured LED2 mode to ChargingError");

    loop {
        let _ = npm1300.enable_led0().await;
        defmt::info!("Enabled LED0");
        Timer::after_millis(1000).await;
        let _ = npm1300.disable_led0().await;
        defmt::info!("Disabled LED0");
        Timer::after_millis(1000).await;
    }

}
