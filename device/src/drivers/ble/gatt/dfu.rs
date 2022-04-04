use crate::{
    actors::dfu::{DfuCommand, FirmwareManager},
    Address,
};
use embedded_storage_async::nor_flash::{AsyncNorFlash, AsyncReadNorFlash};
use heapless::Vec;

// The FirmwareUpdate GATT service
#[nrf_softdevice::gatt_service(uuid = "00001000-b0cd-11ec-871f-d45ddf138840")]
pub struct FirmwareService {
    /// Version of current running firmware
    #[characteristic(uuid = "00001001-b0cd-11ec-871f-d45ddf138840", read)]
    version: Vec<u8, 16>,

    /// Max firmware block size for device
    #[characteristic(uuid = "00001002-b0cd-11ec-871f-d45ddf138840", read)]
    mtu: u8,

    /// State control
    #[characteristic(uuid = "00001003-b0cd-11ec-871f-d45ddf138840", write)]
    control: u8,

    /// Version being written
    #[characteristic(uuid = "00001004-b0cd-11ec-871f-d45ddf138840", write, read)]
    next_version: Vec<u8, 16>,

    /// Current write offset
    #[characteristic(uuid = "00001005-b0cd-11ec-871f-d45ddf138840", read)]
    offset: u32,

    /// Firmware data to be written
    #[characteristic(uuid = "00001006-b0cd-11ec-871f-d45ddf138840", write)]
    firmware: Vec<u8, 64>,
}

pub struct FirmwareGattService<'a, F>
where
    F: AsyncNorFlash + AsyncReadNorFlash + 'static,
{
    service: &'a FirmwareService,
    dfu: Address<FirmwareManager<F>>,
}

impl<'a, F> FirmwareGattService<'a, F>
where
    F: AsyncNorFlash + AsyncReadNorFlash,
{
    pub fn new(
        service: &'a FirmwareService,
        dfu: Address<FirmwareManager<F>>,
        version: &[u8],
        mtu: u8,
    ) -> Result<Self, ()> {
        service
            .version_set(Vec::from_slice(version)?)
            .map_err(|_| ())?;
        service.next_version_set(Vec::new()).map_err(|_| ())?;
        service.offset_set(0).map_err(|_| ())?;
        service.mtu_set(mtu).map_err(|_| ())?;
        Ok(Self { service, dfu })
    }

    pub async fn handle(&self, event: &FirmwareServiceEvent) -> Result<(), ()> {
        match event {
            FirmwareServiceEvent::ControlWrite(value) => {
                debug!("Write firmware control: {}", value);
                if *value == 1 {
                    self.service.offset_set(0).ok();
                    self.dfu
                        .request(DfuCommand::Start)
                        .map_err(|_| ())?
                        .await
                        .map_or(Err(()), |_| Ok(()))?;
                } else if *value == 2 {
                    self.dfu.notify(DfuCommand::Finish).map_err(|_| ())?
                } else if *value == 3 {
                    self.dfu
                        .request(DfuCommand::Booted)
                        .map_err(|_| ())?
                        .await
                        .map_or(Err(()), |_| Ok(()))?;
                }
            }
            FirmwareServiceEvent::FirmwareWrite(value) => {
                let offset = self.service.offset_get().unwrap();
                self.dfu
                    .request(DfuCommand::WriteBlock(value))
                    .map_err(|_| ())?
                    .await
                    .map_or(Err(()), |_| Ok(()))?;
                self.service.offset_set(offset + value.len() as u32).ok();
            }
            _ => {}
        }
        Ok(())
    }
}