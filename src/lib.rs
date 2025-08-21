#![cfg_attr(not(test), no_std)]

use device_driver::AsyncRegisterInterface;

pub mod common;

pub mod adc;
pub mod buck;
pub mod ldsw;
pub mod charger;
pub mod gpios;
pub mod leds;
pub mod pof;
pub mod ship;
pub mod sysreg;

const ADDR: u8 = 0x6B;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum NPM1300Error<I2cError> {
    #[error("i2c error: {0:?}")]
    I2c(I2cError),
    #[error("charger current {0:?} is too high")]
    ChargerCurrentTooHigh(u16),
    #[error("invalid NTC threshold")]
    InvalidNtcThreshold,
    #[error("invalid die temperature stop/resume threshold")]
    InvalidDieTemperatureThreshold,
    #[error("invalid NTC beta")]
    InvalidNtcBeta,
    #[error(
        "invalid VBAT measurement delay value, it must be between 4 and 514 and a multiple of 2"
    )]
    InvalidVbatMeasurementDelayValue,
    #[error("invalid VSYS threshold")]
    InvalidPofVsysThreshold,
    #[error("invalid discharge current value")]
    InvalidDischargeCurrentValue { msb: u8, lsb: u8 },
}

#[derive(Debug)]
pub struct DeviceInterface<I2c: embedded_hal_async::i2c::I2c> {
    pub i2c: I2c,
}

pub struct NPM1300<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs> {
    device: Device<DeviceInterface<I2c>>,
    delay: Delay,
    ntc_beta: Option<f32>,
}

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    NPM1300<I2c, Delay>
{
    pub fn new(i2c: I2c, delay: Delay) -> Self {
        Self {
            device: Device::new(DeviceInterface { i2c }),
            delay,
            ntc_beta: None,
        }
    }
}

device_driver::create_device!(
    device_name: Device,
    manifest: "device.yaml"
);

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface
    for DeviceInterface<I2c>
{
    type AddressType = u16;

    type Error = NPM1300Error<I2c::Error>;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let buf = [(address >> 8) as u8, address as u8, data[0]];
        self.i2c.write(ADDR, &buf).await.map_err(NPM1300Error::I2c)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(ADDR, &[(address >> 8) as u8, address as u8], data)
            .await
            .map_err(NPM1300Error::I2c)
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::AsyncCommandInterface
    for DeviceInterface<I2c>
{
    type AddressType = u16;

    type Error = NPM1300Error<I2c::Error>;

    async fn dispatch_command(
        &mut self,
        address: Self::AddressType,
        size_bits_in: u32,
        input: &[u8],
        _size_bits_out: u32,
        _output: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write_register(address, size_bits_in, input).await
    }
}
