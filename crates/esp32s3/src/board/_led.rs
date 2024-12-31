use anyhow::Result;
use esp_idf_svc::hal::{gpio::OutputPin, prelude::Peripherals, rmt::{config::TransmitConfig, RmtChannel, TxRmtDriver}};

use super::BoardState;

macro_rules! peripheral_selector {
    (
        $struct_name:ident < $($lfdef:lifetime,)+ > {
            $($name:ident: $usage:ident $( + $lifetime:lifetime)?,)+
        }
    ) => {
        paste::paste! {
            pub struct $struct_name <
                // $( $lfdef,)+
                $(
                    [<$name:camel Fn>]: FnOnce(&mut Peripherals) -> &mut [<$name:camel Per>],
                    [<$name:camel Per>]: esp_idf_svc::hal::peripheral::Peripheral < P = [<$name:camel PerUsage>] > /*$(+ $lifetime)?*/,
                    [<$name:camel PerUsage>]: $usage,
                )+
            > {
                $(
                    pub $name: [<$name:camel Fn>],
                )+
            }
        }
    };
}

peripheral_selector! {
    LedPeripheral<'d,> {
        led: OutputPin + 'd,
        channel: RmtChannel + 'd,
    }
}

pub struct Led<'d> {
    led: TxRmtDriver<'d>
}

impl<'d> Led<'d> {
    pub fn new<S: LedPeripheral>(selector: LedPeripheral) -> Result<Self> {
        let led = &mut state.peripherals.pins.gpio2;
        let channel = &mut state.peripherals.rmt.channel0;

        let config = TransmitConfig::new().clock_divider(2);
        let tx = TxRmtDriver::new(channel, led, &config)?;

        Ok(Self { led: tx })
    }
}

