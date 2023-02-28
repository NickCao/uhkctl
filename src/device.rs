use crate::consts::{
    self, ConfigBufferId, DevicePropertyIds, ModulePropertyId, ModuleSlots, UsbVariables,
};
use byteorder::{LittleEndian, ReadBytesExt};
use hidapi::{HidDevice, HidError};
use num_enum::TryFromPrimitiveError;
use std::{cmp::min, io::Read, string::FromUtf8Error, time::Duration};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("hid error")]
    Hid(#[from] HidError),
    #[error("module slot error")]
    LeftSlot(#[from] TryFromPrimitiveError<ModuleSlots>),
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("from utf8 error")]
    Utf8Error(#[from] FromUtf8Error),
}

pub type DeviceResult<T> = Result<T, DeviceError>;

pub struct Device {
    dev: HidDevice,
}

impl Device {
    pub fn open(dev: HidDevice) -> Self {
        // TODO: allow non blocking
        Self { dev }
    }
    pub fn wait(&self) -> DeviceResult<()> {
        while self.state()?.eeprom_busy {
            std::thread::sleep(Duration::from_millis(200));
        }
        Ok(())
    }
    pub fn load_config(&self, buffer: ConfigBufferId) -> DeviceResult<Vec<u8>> {
        let sizes = self.get_config_size()?;
        let size = match buffer {
            ConfigBufferId::HardwareConfig => sizes.0,
            _ => sizes.1,
        } as u16;
        const CHUNK_SIZE: u16 = 63;
        let mut offset: u16 = 0;
        let mut data = vec![];
        while offset < size {
            let reading = min(CHUNK_SIZE, size - offset);
            self.dev.write(&[
                0x0,
                consts::UsbCommand::ReadConfig.into(),
                buffer.into(),
                CHUNK_SIZE as u8,
                offset.to_le_bytes()[0],
                offset.to_le_bytes()[1],
            ])?;
            let mut buf = vec![0u8; consts::MAX_PAYLOAD_SIZE];
            self.dev.read_timeout(&mut buf, 1000)?;
            data.extend_from_slice(&buf[1..(reading + 1) as usize]);
            offset += reading;
        }
        Ok(data)
    }
    pub fn get_module_property(
        &self,
        module: ModuleSlots,
        property: ModulePropertyId,
    ) -> DeviceResult<Vec<u8>> {
        self.dev.write(&[
            0x0,
            consts::UsbCommand::GetModuleProperty.into(),
            module.into(),
            property.into(),
        ])?;
        let mut buf = vec![0u8; consts::MAX_PAYLOAD_SIZE];
        self.dev.read_timeout(&mut buf, 1000)?;
        Ok(buf)
    }
    pub fn get_config_size(&self) -> DeviceResult<(usize, usize)> {
        self.dev.write(&[
            0x0,
            consts::UsbCommand::GetProperty.into(),
            DevicePropertyIds::ConfigSizes.into(),
        ])?;
        let mut buf = vec![0u8; consts::MAX_PAYLOAD_SIZE];
        self.dev.read_timeout(&mut buf, 1000)?;
        Ok((
            (buf[1] as usize + (buf[2] as usize)) << 8,
            (buf[3] as usize + (buf[4] as usize)) << 8,
        ))
    }
    pub fn uptime(&self) -> DeviceResult<Duration> {
        self.dev.write(&[
            0x0,
            consts::UsbCommand::GetProperty.into(),
            DevicePropertyIds::Uptime.into(),
        ])?;
        let mut buf = vec![0u8; consts::MAX_PAYLOAD_SIZE];
        self.dev.read_timeout(&mut buf, 1000)?;
        let mut num = [0u8; 4];
        num.copy_from_slice(&buf[1..5]);
        Ok(Duration::from_millis(u32::from_le_bytes(num).into()))
    }
    pub fn get_variable(&self, var: UsbVariables) -> DeviceResult<u8> {
        self.dev
            .write(&[0x0, consts::UsbCommand::GetVariable.into(), var.into()])?;
        let mut buf = vec![0u8; consts::MAX_PAYLOAD_SIZE];
        self.dev.read_timeout(&mut buf, 1000)?;
        Ok(buf[1])
    }
    #[deprecated]
    pub fn set_test_led(&self, state: bool) -> DeviceResult<()> {
        self.dev.write(&[
            0x0,
            consts::UsbCommand::SetTestLed.into(),
            if state { 1 } else { 0 },
        ])?;
        let mut buf = vec![0u8; consts::MAX_PAYLOAD_SIZE];
        self.dev.read_timeout(&mut buf, 1000)?;
        Ok(())
    }
    #[deprecated]
    pub fn set_brightness(&self, brightness: u8) -> DeviceResult<()> {
        self.dev.write(&[
            0x0,
            consts::UsbCommand::SetLedPwmBrightness.into(),
            brightness,
        ])?;
        let mut buf = vec![0u8; consts::MAX_PAYLOAD_SIZE];
        self.dev.read_timeout(&mut buf, 1000)?;
        Ok(())
    }
    pub fn state(&self) -> DeviceResult<DeviceState> {
        self.dev
            .write(&[0x0, consts::UsbCommand::GetDeviceState.into()])?;
        let mut buf = vec![0u8; consts::MAX_PAYLOAD_SIZE];
        self.dev.read_timeout(&mut buf, 1000)?;
        Ok(DeviceState {
            eeprom_busy: buf[1] != 0,
            halves_merged: buf[2] != 0,
            left_half_connected: buf[3] != 0,
            active_layer: buf[6] & 0x7f,
            active_layer_toggled: buf[6] & 0x80 != 0,
            left_half_slot: buf[3],
            left_module_slot: ModuleSlots::try_from(buf[4])?,
            right_module_slot: ModuleSlots::try_from(buf[5])?,
        })
    }
}

#[derive(Debug)]
pub struct DeviceState {
    pub eeprom_busy: bool,
    pub halves_merged: bool,
    pub left_half_connected: bool,
    pub active_layer: u8,
    pub active_layer_toggled: bool,
    pub left_half_slot: u8,
    pub left_module_slot: ModuleSlots,
    pub right_module_slot: ModuleSlots,
}

pub struct UhkCursor {
    cursor: std::io::Cursor<Vec<u8>>,
}

impl UhkCursor {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            cursor: std::io::Cursor::new(data),
        }
    }
    pub fn read_bool(&mut self) -> DeviceResult<bool> {
        Ok(self.read_u8()? != 0)
    }
    pub fn read_u8(&mut self) -> DeviceResult<u8> {
        Ok(self.cursor.read_u8()?)
    }
    pub fn read_u16(&mut self) -> DeviceResult<u16> {
        Ok(self.cursor.read_u16::<LittleEndian>()?)
    }
    pub fn read_u32(&mut self) -> DeviceResult<u32> {
        Ok(self.cursor.read_u32::<LittleEndian>()?)
    }
    pub fn read_compact_length(&mut self) -> DeviceResult<u16> {
        let length = self.read_u8()?;
        if length == 0xff {
            Ok(self.read_u16()?)
        } else {
            Ok(length.into())
        }
    }
    pub fn read_string(&mut self) -> DeviceResult<String> {
        let length = self.read_compact_length()?;
        dbg!(length);
        let mut buf = vec![0u8; length as usize];
        self.cursor.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
}
