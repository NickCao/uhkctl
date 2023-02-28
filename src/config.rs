use std::io::Read;

use crate::device::{DeviceResult, UhkCursor};

#[derive(Debug)]
pub struct HardwareConfig {
    pub signature: String,
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub brand_id: u8,
    pub device_id: u8,
    pub unique_id: u32,
    pub vendor_mode: bool,
    pub iso_mode: bool,
}

impl HardwareConfig {
    pub fn deserialize(cursor: &mut UhkCursor) -> DeviceResult<Self> {
        let signature = cursor.read_string()?;
        let major = cursor.read_u8()?;
        let minor = cursor.read_u8()?;
        let patch = cursor.read_u8()?;
        let brand_id = cursor.read_u8()?;
        let device_id = cursor.read_u8()?;
        let unique_id = cursor.read_u32()?;
        let vendor_mode = cursor.read_bool()?;
        let iso_mode = cursor.read_bool()?;
        Ok(Self {
            signature,
            major,
            minor,
            patch,
            brand_id,
            device_id,
            unique_id,
            vendor_mode,
            iso_mode,
        })
    }
}
