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

#[derive(Debug)]
pub struct MouseConfig {
    pub move_initial_speed: u8,
    pub move_acceleration: u8,
    pub move_decelerated_speed: u8,
    pub move_base_speed: u8,
    pub move_accelerated_speed: u8,
    pub scroll_initial_speed: u8,
    pub scroll_acceleration: u8,
    pub scroll_decelerated_speed: u8,
    pub scroll_base_speed: u8,
    pub scroll_accelerated_speed: u8,
}

impl MouseConfig {
    fn deserialize(cursor: &mut UhkCursor) -> DeviceResult<Self> {
        let move_initial_speed = cursor.read_u8()?;
        let move_acceleration = cursor.read_u8()?;
        let move_decelerated_speed = cursor.read_u8()?;
        let move_base_speed = cursor.read_u8()?;
        let move_accelerated_speed = cursor.read_u8()?;
        let scroll_initial_speed = cursor.read_u8()?;
        let scroll_acceleration = cursor.read_u8()?;
        let scroll_decelerated_speed = cursor.read_u8()?;
        let scroll_base_speed = cursor.read_u8()?;
        let scroll_accelerated_speed = cursor.read_u8()?;
        Ok(Self {
            move_initial_speed,
            move_acceleration,
            move_decelerated_speed,
            move_base_speed,
            move_accelerated_speed,
            scroll_initial_speed,
            scroll_acceleration,
            scroll_decelerated_speed,
            scroll_base_speed,
            scroll_accelerated_speed,
        })
    }
}

#[derive(Debug)]
pub struct UserConfig {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,

    pub length: u16,
    pub name: String,

    pub double_tap_switch_layer_timeout: u16,
    pub icons_and_layer_texts_brightness: u8,
    pub alphanumeric_segments_brighrness: u8,
    pub key_backlight_brightness: u8,

    pub mouse_config: MouseConfig,
}

impl UserConfig {
    pub fn deserialize(cursor: &mut UhkCursor) -> DeviceResult<Self> {
        let major = cursor.read_u16()?;
        let minor = cursor.read_u16()?;
        let patch = cursor.read_u16()?;

        let length = cursor.read_u16()?;
        let name = cursor.read_string()?;

        let double_tap_switch_layer_timeout = cursor.read_u16()?;
        let icons_and_layer_texts_brightness = cursor.read_u8()?;
        let alphanumeric_segments_brighrness = cursor.read_u8()?;
        let key_backlight_brightness = cursor.read_u8()?;

        let mouse_config = MouseConfig::deserialize(cursor)?;

        Ok(Self {
            major,
            minor,
            patch,
            length,
            name,
            double_tap_switch_layer_timeout,
            icons_and_layer_texts_brightness,
            alphanumeric_segments_brighrness,
            key_backlight_brightness,
            mouse_config,
        })
    }
}
