#![no_std]
#![no_main]

//! Example demonstrating control of the NPM1300 PMIC's BUCK2 voltage regulator

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    peripherals,
    twim::{self, Twim},
};
use embassy_time::Timer;

use {defmt_rtt as _, panic_probe as _};

use npm1300_rs::{
    buck::BuckVoltage,
    gpios::{Gpio, GpioPolarity},
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

    let config = twim::Config::default();
    let mut pmic_gpio0 = Output::new(p.P0_11, Level::Low, OutputDrive::Standard);
    let mut pmic_gpio1 = Output::new(p.P0_12, Level::High, OutputDrive::Standard);

    let twi = Twim::new(p.SERIAL0, Irqs, sdapin, sclpin, config);

    let mut npm1300 = NPM1300::new(twi, embassy_time::Delay);
    defmt::info!("Enabling buck 2...");
    let _ = npm1300.enable_buck2().await;

    let buck_status = npm1300.get_buck_status().await;
    defmt::info!("Buck status: {:?}", buck_status);

    defmt::info!("Waiting 5s...");
    Timer::after_millis(5000).await;

    defmt::info!("Setting buck 2 voltage to 1.8V...");
    let _ = npm1300.set_buck2_normal_voltage(BuckVoltage::V1_8).await;
    let buck2_current_voltage = npm1300.get_buck2_vout_status().await;
    defmt::info!("Set buck 2 voltage to {}", buck2_current_voltage);

    defmt::info!("Waiting 5s...");
    Timer::after_millis(5000).await;

    defmt::info!("Setting buck 2 retention voltage to 2.5V...");
    let _ = npm1300
        .configure_buck2_retention_mode(BuckVoltage::V2_5, Gpio::Gpio0, GpioPolarity::NotInverted)
        .await;
    defmt::info!("Waiting 5s...");
    Timer::after_millis(5000).await;

    defmt::info!("Enabling PMIC GPIO0...");
    pmic_gpio0.set_high();
    let buck2_current_voltage = npm1300.get_buck2_vout_status().await;
    defmt::info!("Set buck 2 voltage to {}", buck2_current_voltage);

    defmt::info!("Waiting 5s...");
    Timer::after_millis(5000).await;

    defmt::info!("Disabling PMIC GPIO0...");
    pmic_gpio0.set_low();

    defmt::info!("Waiting 5s...");
    Timer::after_millis(5000).await;

    defmt::info!("Configuring buck 2 GPIO enable control...");
    let _ = npm1300
        .set_buck2_gpio_control(Gpio::Gpio1, GpioPolarity::NotInverted)
        .await;

    defmt::info!("Waiting 5s...");
    Timer::after_millis(5000).await;

    defmt::info!("Disabling PMIC GPIO1...");
    pmic_gpio1.set_low();

    defmt::info!("Waiting 5s...");
    Timer::after_millis(5000).await;

    defmt::info!("Enabling PMIC GPIO1...");
    pmic_gpio1.set_high();

    defmt::info!("Disabling buck 2...");
    let _ = npm1300.disable_buck2().await;

    loop {
        Timer::after_millis(1000).await;
    }
}
