use std::iter;

use anyhow::Result;
use esp_idf_svc::hal::{gpio::OutputPin, peripheral::Peripheral, rmt::RmtChannel};
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrbw32, LedPixelEsp32Rmt, RGBW8};

use smart_leds_trait::SmartLedsWrite;

pub struct Led<'d> {
    driver: LedPixelEsp32Rmt<'d, RGBW8, LedPixelColorGrbw32>,
}

impl<'d> Led<'d> {
    pub fn new(
        led: impl Peripheral<P = impl OutputPin> + 'd,
        channel: impl Peripheral<P = impl RmtChannel> + 'd,
    ) -> Result<Self> {
        Ok(Self {
            driver: LedPixelEsp32Rmt::new(channel, led)?
        })
    }

    pub async fn set_pixel(&mut self, color: RGBW8) -> Result<()> {
        let pixels = iter::repeat(color).take(25);
        self.driver.write(pixels)?;
        Ok(())
    }
}

