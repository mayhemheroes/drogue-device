use crate::bsp::Board;
use crate::drivers::{
    button::Button,
    led::{ActiveHigh, ActiveLow, Led},
};
use embassy_nrf::{
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
    peripherals::{
        P0_02, P0_03, P0_04, P0_05, P0_06, P0_07, P0_08, P0_09, P0_10, P0_11, P0_12, P0_13, P0_14,
        P0_15, P0_16, P0_24, P0_25, P0_26, P0_27, P0_28, P0_29, P0_30, P0_31, P1_02, P1_08, P1_09,
        PWM0, UARTE0, USBD,
    },
};

pub type PinLedRed = Output<'static, AnyPin>;
pub type LedRed = Led<PinLedRed, ActiveHigh>;

pub type PinLedBlue = Output<'static, AnyPin>;
pub type LedBlue = Led<PinLedBlue, ActiveHigh>;

pub type PinSwitch = Input<'static, P1_02>;
pub type Switch = Button<PinSwitch, ActiveLow>;

pub struct AdafruitFeatherNrf52840 {
    pub uarte0: UARTE0,
    pub pwm0: PWM0,
    pub usbd: USBD,
    pub red_led: PinLedRed,
    pub blue_led: PinLedBlue,
    pub switch: PinSwitch,
    pub d2: P0_10,
    pub tx: P0_25,
    pub rx: P0_24,
    pub miso: P0_15,
    pub mosi: P0_13,
    pub sck: P0_14,
    pub a5: P0_03,
    pub a4: P0_02,
    pub a3: P0_28,
    pub a2: P0_30,
    pub a1: P0_05,
    pub a0: P0_04,
    pub aref: P0_31,
    pub sda: P0_12,
    pub scl: P0_11,
    pub d5: P1_08,
    pub d6: P0_07,
    pub d9: P0_26,
    pub d10: P0_27,
    pub d11: P0_06,
    pub d12: P0_08,
    pub d13: P1_09,
    pub neopixel: P0_16,
    pub nfc1: P0_09,
    pub voltage: P0_29,
}

impl Board for AdafruitFeatherNrf52840 {
    type Peripherals = embassy_nrf::Peripherals;
    type BoardConfig = ();

    fn new(p: Self::Peripherals) -> Self {
        Self {
            uarte0: p.UARTE0,
            pwm0: p.PWM0,
            usbd: p.USBD,
            red_led: Output::new(p.P1_15.degrade(), Level::Low, OutputDrive::Standard),
            blue_led: Output::new(p.P1_10.degrade(), Level::Low, OutputDrive::Standard),
            switch: Input::new(p.P1_02, Pull::Up),

            d2: p.P0_10,
            tx: p.P0_25,
            rx: p.P0_24,
            miso: p.P0_15,
            mosi: p.P0_13,
            sck: p.P0_14,
            a5: p.P0_03,
            a4: p.P0_02,
            a3: p.P0_28,
            a2: p.P0_30,
            a1: p.P0_05,
            a0: p.P0_04,
            aref: p.P0_31,
            sda: p.P0_12,
            scl: p.P0_11,
            d5: p.P1_08,
            d6: p.P0_07,
            d9: p.P0_26,
            d10: p.P0_27,
            d11: p.P0_06,
            d12: p.P0_08,
            d13: p.P1_09,
            neopixel: p.P0_16,
            nfc1: p.P0_09,
            voltage: p.P0_29,
        }
    }
}