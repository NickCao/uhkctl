#![feature(cstr_from_bytes_until_nul)]
use anyhow::Result;
use hidapi::HidApi;
use uhkctl::{
    config::{HardwareConfig, UserConfig},
    consts::{
        KeyActionId, KeystrokeActionFlag, KeystrokeType, MacroActionId, ModuleSlots, UsbVariables,
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
        for j in 0..action_length {
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
            // do peek
        }
    }

    let num_keymaps = cursor.read_compact_length().unwrap();
    dbg!(num_keymaps);
    for _ in 0..num_keymaps {
        dbg!(cursor.read_string().unwrap()); // abbr
        dbg!(cursor.read_bool().unwrap()); // default
        dbg!(cursor.read_string().unwrap()); // name
        dbg!(cursor.read_string().unwrap()); // desc
        let num_layers = cursor.read_compact_length().unwrap();
        dbg!(num_layers);
        for _ in 0..num_layers {
            dbg!(cursor.read_u8().unwrap()); // id
            let num_m = cursor.read_compact_length().unwrap();
            for _ in 0..num_m {
                dbg!("module");
                dbg!(cursor.read_u8().unwrap()); // id
                let ka_len = cursor.read_compact_length().unwrap();
                dbg!(ka_len);
                for _ in 0..ka_len {
                    let ka_id = cursor.read_u8().unwrap(); // ka id
                    dbg!(ka_id);
                    match ka_id {
                        _ if ka_id >= KeyActionId::KeystrokeAction.into()
                            && ka_id < KeyActionId::LastKeystrokeAction.into() =>
                        {
                            dbg!("key");
                            let flags = ka_id - u8::from(KeyActionId::NoneAction);
                            dbg!(flags);
                            let ty = flags >> 3 & 0b11;
                            dbg!(ty);
                            if flags & u8::from(KeystrokeActionFlag::Scancode) != 0 {
                                let scancode = if ty == KeystrokeType::LongMedia.into() {
                                    cursor.read_u16().unwrap()
                                } else {
                                    cursor.read_u8().unwrap() as u16
                                };
                                dbg!(scancode);
                            }
                            if flags & u8::from(KeystrokeActionFlag::ModifierMask) != 0 {
                                let mask = cursor.read_u8().unwrap();
                                dbg!(mask);
                            }
                            if flags & u8::from(KeystrokeActionFlag::SecondaryRoleAction) != 0 {
                                let role = cursor.read_u8().unwrap();
                                dbg!(role);
                            }
                        }
                        _ if ka_id == KeyActionId::NoneAction.into() => (),
                        _ if ka_id == KeyActionId::SwitchLayerAction.into() => {
                            dbg!("switch layer");
                            dbg!(cursor.read_u8().unwrap()); // layer
                            dbg!(cursor.read_u8().unwrap()); // mode
                        }
                        _ if ka_id == KeyActionId::SwitchKeymapAction.into() => {
                            dbg!("switch keymap");
                            dbg!(cursor.read_u8().unwrap()); // keymap
                        }
                        _ if ka_id == KeyActionId::MouseAction.into() => {
                            dbg!("mouse");
                            dbg!(cursor.read_u8().unwrap()); // layer
                        }
                        _ => {
                            dbg!(ka_id);
                            unimplemented!()
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
