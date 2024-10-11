//! Turbidimeter (TSW-30, KIE TS-300B)
//!
//! * Turbidity sensor TS-300B measures the turbidity (counter of suspended
//!   matter) in the wash water. Based on the optical principle, the sensor uses
//!   the light-emitting diode and phototransistor to receive the specific
//!   wavelength to measure the opacity or other substances of dirty water.
//! * The working principle the turbidity sensor: When light passes through a
//!   certain amount of water, the amount of light penetration depends on the
//!   amount of dirt in the water. When the amount of dirt increases, the light
//!   that penetrates the water sample decreases, and the turbidity sensor
//!   measures the amount of light transmitted to calculate the turbidity of the
//!   wash water.
//! * Ratio Range (NTU): 0~1000±30；Output Voltage: 0~5V；Reverse Voltage: 5V
//! * Infrared Emitting Diode(Peak emission wavelength): 940nm
//! * Photo Transistor(Peak emission wavelength): 880nm
//!
//! Датчик мутности (непрозрачности, затемненности) жидкости, состоит из
//! передатчика и приемника. Передатчик - источника света, обычно светодиод,
//! приемник детектор света, обычно фотодиод или фоторезистор. Измеряемая
//! жидкость (раствор) находится между. Мутность жидкости получаем, измеряя
//! интенсивность света между приемником и передатчиком. Интенсивность принятого
//! света обратно пропорциональна мутности жидкости.
//!
//! * https://aliexpress.ru/item/4000460829861.html?sku_id=12000037978291569&spm=a2g2w.productlist.search_results.0.2e6f7694Qn4bqo
//! * https://dzen.ru/a/Zlaw2DXb4GG2ouqQ

use esp_idf_svc::{
    hal::{
        adc::{
            attenuation::DB_11,
            oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
            Adc,
            Resolution::Resolution12Bit,
        },
        gpio::ADCPin,
        peripheral::Peripheral,
    },
    sys::EspError,
};

/// Turbidimeter (Spectrophotometer)
pub struct Turbidimeter<'a, T: ADCPin> {
    driver: AdcChannelDriver<'a, T, AdcDriver<'a, <T as ADCPin>::Adc>>,
}

impl<'a, T: ADCPin> Turbidimeter<'a, T> {
    pub fn new(
        adc: impl Peripheral<P = <T as ADCPin>::Adc> + 'a,
        pin: impl Peripheral<P = T> + 'a,
    ) -> Result<Self, EspError> {
        let driver = AdcDriver::new(adc)?;
        Ok(Self {
            driver: AdcChannelDriver::new(
                driver,
                pin,
                &AdcChannelConfig {
                    attenuation: DB_11,
                    resolution: Resolution12Bit,
                    calibration: true,
                },
            )?,
        })
    }

    pub fn read(&mut self) -> Result<u16, EspError> {
        self.driver.read()
    }
}
