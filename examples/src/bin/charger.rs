#![no_std]
#![no_main]

//! Example demonstrating charger control of the nPM1300 PMIC

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts, peripherals,
    twim::{self, Twim},
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use npm1300_rs::{leds::LedMode, NPM1300};

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let config = twim::Config::default();

    let twi = Twim::new(p.TWISPI0, Irqs, p.P0_07, p.P0_12, config);

    let mut npm1300 = NPM1300::new(twi);

    // Configure and enable LED0
    defmt::info!("Enabling LED0...");
    npm1300.configure_led0_mode(LedMode::Host).await.unwrap();
    npm1300.enable_led0().await.unwrap();

    defmt::info!("Ignoring NTC measurements...");
    npm1300.ignore_ntc_measurements().await.unwrap();

    defmt::info!("Configuring USB...");
    npm1300
        .set_vbus_in_startup_current_limit(npm1300_rs::sysreg::VbusInCurrentLimit::MA1000)
        .await
        .unwrap();

    defmt::info!("Configuring charger...");
    npm1300.set_charger_current(800).await.unwrap();
    npm1300
        .set_warm_temperature_termination_voltage(
            npm1300_rs::charger::ChargerTerminationVoltage::V4_20,
        )
        .await
        .unwrap();
    npm1300
        .set_normal_temperature_termination_voltage(
            npm1300_rs::charger::ChargerTerminationVoltage::V4_20,
        )
        .await
        .unwrap();

    defmt::info!("Enabling charging...");
    let _ = npm1300.enable_battery_charging().await;

    loop {
        Timer::after_millis(3000).await;
        let status = npm1300.get_charger_status().await;
        defmt::info!("Charger status: {:?}", status);
        let status = npm1300.get_charger_error_reason_and_sensor_value().await;
        defmt::info!("Charger error reason and sensor value: {:?}", status);
        let status = npm1300.get_vbus_in_status().await;
        defmt::info!("VBUS in status: {:?}", status);
    }
}
