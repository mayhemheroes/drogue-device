use stm32l4xx_hal::{
    pac::{I2C2},
    gpio::{
        PA5,
        Output,
        PushPull,
        PC13,
        Input,
        PullUp,
        OpenDrain,
        AF4,
        Alternate,
        PB10,
        PB11,
        PD15,
        PullDown,
        PB14,
    },
    i2c::I2c,
    pac::TIM15,
};
use drogue_device::{
    prelude::*,
    synchronization::Mutex,
    driver::{
        sensor::hts221::{
            Hts221,
        },
        led::{
            SimpleLED,
            Blinker,
        },
        button::{
            Button,
            ButtonEvent,
        },
        timer::Timer,
    },
    hal::timer::stm32l4xx::Timer as McuTimer,
};
use drogue_device::driver::sensor::hts221::SensorAcquisition;
use drogue_device::domain::time::duration::Milliseconds;

type Ld1Actor = SimpleLED<PA5<Output<PushPull>>>;
type Ld2Actor = SimpleLED<PB14<Output<PushPull>>>;
type ButtonInterrupt = Button<MyDevice, PC13<Input<PullUp>>>;

type I2cScl = PB10<Alternate<AF4, Output<OpenDrain>>>;
type I2cSda = PB11<Alternate<AF4, Output<OpenDrain>>>;
type I2cPeriph = I2c<I2C2, (I2cScl, I2cSda)>;
type I2cActor = Mutex<I2cPeriph>;

type Blinker1Actor = Blinker<PA5<Output<PushPull>>, McuTimer<TIM15>>;
type Blinker2Actor = Blinker<PB14<Output<PushPull>>, McuTimer<TIM15>>;

type TimerActor = Timer<McuTimer<TIM15>>;

type Hts221Package = Hts221<MyDevice, PD15<Input<PullDown>>, I2cPeriph>;
//type Hts221Sensor = Sensor<MyDevice, I2cPeriph>;

pub struct MyDevice {
    pub ld1: ActorContext<Ld1Actor>,
    pub ld2: ActorContext<Ld2Actor>,
    pub blinker1: ActorContext<Blinker1Actor>,
    pub blinker2: ActorContext<Blinker2Actor>,
    pub button: InterruptContext<ButtonInterrupt>,
    pub i2c: ActorContext<I2cActor>,
    pub hts221: Hts221Package,
    pub timer: InterruptContext<TimerActor>,
}

impl Device for MyDevice {
    fn mount(&'static mut self, bus_address: &Address<EventBus<Self>>, supervisor: &mut Supervisor) {
        let ld1_addr = self.ld1.mount(supervisor);
        let ld2_addr = self.ld2.mount(supervisor);

        let blinker1_addr = self.blinker1.mount(supervisor);
        let blinker2_addr = self.blinker2.mount(supervisor);

        let i2c_addr = self.i2c.mount(supervisor);
        let hts221_addr = self.hts221.mount(bus_address, supervisor);
        let timer_addr = self.timer.mount(supervisor);

        blinker1_addr.bind(&timer_addr);
        blinker1_addr.bind(&ld1_addr);

        blinker2_addr.bind(&timer_addr);
        blinker2_addr.bind(&ld2_addr);


        hts221_addr.bind(&i2c_addr);

        let button_addr = self.button.mount(supervisor);
        button_addr.bind(bus_address);
    }
}

impl EventConsumer<ButtonEvent> for MyDevice {
    fn on_event(&'static mut self, message: ButtonEvent) where
        Self: Sized, {
        match message {
            ButtonEvent::Pressed => {
                log::info!("[event-bus] button pressed");
                self.blinker1.address().adjust_delay(Milliseconds(100u32));
            }
            ButtonEvent::Released => {
                log::info!("[event-bus] button released");
                self.blinker1.address().adjust_delay(Milliseconds(500u32));
            }
        }
    }
}

impl EventConsumer<SensorAcquisition> for MyDevice {
    fn on_event(&'static mut self, message: SensorAcquisition)
        where
            Self: Sized, {
        log::info!("[event-bus] {:?}", message);
    }
}

