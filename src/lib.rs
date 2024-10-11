//! Turbidimeter (TSW-30, KIE TS-300B)
use esp_idf_svc::{
    hal::{
        adc::{
            attenuation::DB_11, Adc, AdcContConfig, AdcContDriver, AdcMeasurement, Attenuated,
            Resolution::Resolution12Bit,
        },
        gpio::ADCPin,
        peripheral::{Peripheral, PeripheralRef},
    },
    sys::EspError,
};
use std::{marker::PhantomData, slice::from_mut};

// https://github.com/esp-rs/esp-idf-hal/blob/4f4478718e88344082b82af455192ba10efd41c8/src/adc.rs#L915

/// Turbidimeter (Spectrophotometer)
pub struct Turbidimeter<'a> {
    pub driver: AdcContDriver<'a>,
    // _adc: PeripheralRef<'d, ADC>,
}

impl<'a> Turbidimeter<'a> {
    pub fn new<T: ADCPin>(
        adc: impl Peripheral<P = <T as ADCPin>::Adc> + 'a,
        pin: impl Peripheral<P = T> + 'a,
    ) -> Result<Self, EspError> {
        let mut driver = AdcContDriver::new(adc, &AdcContConfig::default(), Attenuated::db11(pin))?;
        Ok(Self { driver })
    }

    pub async fn read<const N: usize>(&mut self) -> Result<[u16; N], EspError> {
        let mut measurements = [AdcMeasurement::new(); N];
        self.driver.read_async(&mut measurements).await?;
        Ok(measurements.map(|measurement| measurement.data()))
    }
}
