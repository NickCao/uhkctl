#![feature(cstr_from_bytes_until_nul)]
use anyhow::Result;
use hidapi::HidApi;
use uhkctl::{
    config::{HardwareConfig, KeymapConfig, UserConfig},
    consts::{
        MacroActionId, ModuleSlots, UsbVariables,
    },
    device::{Device, UhkCursor},
};

fn main() -> Result<()> {
    env_logger::init();
    let api = HidApi::new()?;
    let devices = uhkctl::devices(&api);
    let info = devices.first().unwrap();
    let device = info.open_device(&api).unwrap();
    let device = Device::open(device);
    dbg!(device.state().unwrap());
    dbg!(device.uptime().unwrap());
    dbg!(device.get_variable(UsbVariables::TestSwitches).unwrap());
    dbg!(device.get_variable(UsbVariables::TestUsbStack).unwrap());
    dbg!(device
        .get_variable(UsbVariables::DebounceTimePress)
        .unwrap());
    dbg!(device
        .get_variable(UsbVariables::DebounceTimeRelease)
        .unwrap());
    dbg!(device
        .get_variable(UsbVariables::UsbReportSemaphore)
        .unwrap());
    dbg!(device.get_config_size().unwrap());
    let p = device
        .get_module_property(
            ModuleSlots::LeftKeyboardHalf,
            uhkctl::consts::ModulePropertyId::GitTag,
        )
        .unwrap();
    dbg!(std::ffi::CStr::from_bytes_until_nul(&p[1..]).unwrap());
    let p = device
        .get_module_property(
            ModuleSlots::LeftKeyboardHalf,
            uhkctl::consts::ModulePropertyId::GitRepo,
        )
        .unwrap();
    dbg!(std::ffi::CStr::from_bytes_until_nul(&p[1..]).unwrap());
    let cfg = device
        .load_config(uhkctl::consts::ConfigBufferId::HardwareConfig)
        .unwrap();
    let mut cursor = UhkCursor::new(cfg);
    dbg!(HardwareConfig::deserialize(&mut cursor).unwrap());

    let cfg = device
        .load_config(uhkctl::consts::ConfigBufferId::ValidatedUserConfig)
        .unwrap();
    let mut cursor = UhkCursor::new(cfg);
    dbg!(UserConfig::deserialize(&mut cursor).unwrap());

    let num_modules = cursor.read_compact_length().unwrap();
    dbg!(num_modules);
    for i in 0..num_modules {
        dbg!(i);
        dbg!(cursor.read_u8().unwrap()); // id
        dbg!(cursor.read_u8().unwrap()); // pointerMode
        dbg!(cursor.read_u8().unwrap()); // deceleratedPointerSpeedMultiplier
        dbg!(cursor.read_u8().unwrap()); // basePointerSpeedMultiplier
        dbg!(cursor.read_u8().unwrap()); // acceleratedPointerSpeedMultiplier
        dbg!(cursor.read_u16().unwrap()); // angularShift
        dbg!(cursor.read_u8().unwrap()); // modLayerPointerFunction
        dbg!(cursor.read_u8().unwrap()); // fnLayerPointerFunction
        dbg!(cursor.read_u8().unwrap()); // mouseLayerPointerFunction
    }

    let num_macros = cursor.read_compact_length().unwrap();
    dbg!(num_macros);
    for i in 0..num_macros {
        dbg!(i);
        dbg!(cursor.read_bool().unwrap()); // is looped
        dbg!(cursor.read_bool().unwrap()); // is private
        dbg!(cursor.read_string().unwrap()); // name
        let action_length = cursor.read_compact_length().unwrap();
        dbg!(action_length);
        for _j in 0..action_length {
            let id = cursor.read_u8().unwrap();
            dbg!(id);
            match id {
                _ if id >= MacroActionId::KeyMacroAction.into()
                    && id <= MacroActionId::LastKeyMacroAction.into() =>
                {
                    unimplemented!()
                }
                _ if id >= MacroActionId::MouseButtonMacroAction.into()
                    && id <= MacroActionId::LastMouseButtonMacroAction.into() =>
                {
                    unimplemented!()
                }
                _ if id == MacroActionId::CommandMacroAction.into() => {
                    dbg!(cursor.read_string().unwrap()); // command
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }

    let num_keymaps = cursor.read_compact_length().unwrap();
    dbg!(num_keymaps);
    for _ in 0..num_keymaps {
        dbg!(KeymapConfig::deserialize(&mut cursor).unwrap());
    }

    Ok(())
}
