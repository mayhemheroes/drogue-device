#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use defmt_rtt as _;
use panic_probe as _;

use drogue_device::{
    bind_bsp,
    bsp::{boards::stm32h7::nucleo_h743zi::*, Board},
    domain::temperature::Celsius,
    drivers::tcp::smoltcp::*,
    network::tcp::*,
    DeviceContext,
};
use drogue_temperature::*;
use embassy::util::Forever;
use embassy_net::StaticConfigurator;
use embassy_net::{Config as NetConfig, Ipv4Address, Ipv4Cidr, StackResources};
use embassy_stm32::peripherals::RNG;
use embassy_stm32::rng::Rng;
use embassy_stm32::Peripherals;
use heapless::Vec;

type SmolTcp = SmolTcpStack<'static, 1, 2, 1024>;

// Creates a newtype named `BSP` around the `NucleoH743` to avoid
// orphan rules and apply delegation boilerplate.
bind_bsp!(NucleoH743, BSP);

impl TemperatureBoard for BSP {
    type Network = SharedTcpStack<'static, SmolTcp>;
    type TemperatureScale = Celsius;
    type SensorReadyIndicator = AlwaysReady;
    type Sensor = FakeSensor;
    type SendTrigger = UserButton;
    type Rng = TlsRand;
}

static DEVICE: DeviceContext<TemperatureDevice<BSP>> = DeviceContext::new();
static ETH: Forever<EthernetDevice> = Forever::new();
static CONFIG: Forever<StaticConfigurator> = Forever::new();
static RESOURCES: Forever<StackResources<1, 2, 8>> = Forever::new();

#[embassy::task]
async fn net() {
    embassy_net::run().await
}

#[embassy::main]
async fn main(spawner: embassy::executor::Spawner, p: Peripherals) {
    let board = NucleoH743::new(p);

    unsafe {
        RNG_INST.replace(board.rng);
    }

    let config = CONFIG.put(StaticConfigurator::new(NetConfig {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 0, 111), 24),
        dns_servers: Vec::new(),
        gateway: Some(Ipv4Address::new(192, 168, 0, 1)),
    }));

    let device = ETH.put(board.eth);
    let resources = RESOURCES.put(StackResources::new());
    embassy_net::init(device, config, resources);

    static NETWORK: TcpStackState<SmolTcp> = TcpStackState::new();
    let network = NETWORK.initialize(SmolTcp::new());

    DEVICE
        .configure(TemperatureDevice::new())
        .mount(
            spawner,
            TlsRand,
            TemperatureBoardConfig {
                send_trigger: board.user_button,
                sensor: FakeSensor(22.0),
                sensor_ready: AlwaysReady,
                network,
            },
        )
        .await;
    defmt::info!("Application initialized. Press the blue button to send data");
}

static mut RNG_INST: Option<Rng<RNG>> = None;

#[no_mangle]
fn _embassy_rand(buf: &mut [u8]) {
    use rand_core::RngCore;

    critical_section::with(|_| unsafe {
        defmt::unwrap!(RNG_INST.as_mut()).fill_bytes(buf);
    });
}

pub struct TlsRand;

impl rand_core::RngCore for TlsRand {
    fn next_u32(&mut self) -> u32 {
        critical_section::with(|_| unsafe { defmt::unwrap!(RNG_INST.as_mut()).next_u32() })
    }
    fn next_u64(&mut self) -> u64 {
        critical_section::with(|_| unsafe { defmt::unwrap!(RNG_INST.as_mut()).next_u64() })
    }
    fn fill_bytes(&mut self, buf: &mut [u8]) {
        critical_section::with(|_| unsafe {
            defmt::unwrap!(RNG_INST.as_mut()).fill_bytes(buf);
        });
    }
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), rand_core::Error> {
        critical_section::with(|_| unsafe {
            defmt::unwrap!(RNG_INST.as_mut()).fill_bytes(buf);
        });
        Ok(())
    }
}
impl rand_core::CryptoRng for TlsRand {}
