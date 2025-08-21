#![no_std]
#![no_main]

//! Example demonstrating the NPM1300 PMIC's vbat measurement and temp features

use embassy_executor::Spawner;
use embassy_nrf::{
    peripherals,
    bind_interrupts,
    twim::{self, Twim},
};

use embassy_time::Timer;

use {defmt_rtt as _, panic_probe as _};

use npm1300_rs::{
    NtcThermistorType,
    charger::{
        ChargerTerminationVoltage,
        DischargeCurrentLimit,
        ChargerTerminationCurrentLevelSelect,
    },
    sysreg::{
        VbusInCurrentLimit,
    },
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

    // Modify the configuration fields
    config.sda_pullup = true;
    config.scl_pullup = true;

    defmt::info!("Configuring TWIM...");
    let twi = Twim::new(p.SERIAL0, Irqs, sdapin, sclpin, config);
    
    let mut npm1300 = NPM1300::new(twi, embassy_time::Delay);
    
    let _ = npm1300.set_vbus_in_current_limit(VbusInCurrentLimit::MA1000).await;

    
    defmt::info!("Configuring NTC Resistor...");
    let _ = npm1300.configure_ntc_resistance(NtcThermistorType::Ntc10K, Some(3380.0)).await;
    let _ = npm1300.use_ntc_measurements().await;

    defmt::info!("Configuring Charging...");
    let _ = npm1300.set_charger_current(300).await;
    let _ = npm1300.set_termination_current_level(ChargerTerminationCurrentLevelSelect::SEL10).await;
    let _ = npm1300.set_normal_temperature_termination_voltage(ChargerTerminationVoltage::V4_20).await;
    let _ = npm1300.set_warm_temperature_termination_voltage(ChargerTerminationVoltage::V4_10).await;
    let _ = npm1300.set_discharge_current_limit(DischargeCurrentLimit::Low).await;
    let _ = npm1300.enable_battery_charging().await;
    let _ = npm1300.enable_battery_recharge().await;
    let _ = npm1300.configure_ibat_measurement(true).await;

    loop{
        let vbat_voltage = npm1300.measure_vbat().await.unwrap();
        let _ = npm1300.measure_ntc().await;
        let ntc_temp = npm1300.get_ntc_measurement_result().await.unwrap();
        let die_temp = npm1300.measure_die_temperature().await;
        defmt::info!("Setup: {:?}", npm1300.get_charger_status().await.unwrap());
        defmt::info!("VBAT: {=f32} mV, NTC Temp: {=f32}, Die Temp: {:?} C", vbat_voltage, ntc_temp, die_temp);
        let ibat_current = npm1300.measure_ibat().await.unwrap();
        defmt::info!("IBAT Current: {:?}", ibat_current);
        Timer::after_millis(1000).await;
    }
}
