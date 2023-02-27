use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const MAX_PAYLOAD_SIZE: usize = 64;

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum ModuleSlots {
    NoModule = 0,
    LeftKeyboardHalf = 1,
    KeyClusterLeft = 2,
    TrackballRight = 3,
    TrackpointRight = 4,
    TouchpadRight = 5,
}

#[derive(IntoPrimitive)]
#[repr(u8)]
pub enum UsbCommand {
    GetProperty = 0x00,
    Reenumerate = 0x01,
    JumpToModuleBootloader = 0x02,
    SendKbootCommandToModule = 0x03,
    ReadConfig = 0x04,
    WriteHardwareConfig = 0x05,
    WriteStagingUserConfig = 0x06,
    ApplyConfig = 0x07,
    LaunchEepromTransfer = 0x08,
    GetDeviceState = 0x09,
    SetTestLed = 0x0a,
    GetDebugBuffer = 0x0b,
    GetAdcValue = 0x0c,
    SetLedPwmBrightness = 0x0d,
    GetModuleProperty = 0x0e,
    GetSlaveI2cErrors = 0x0f,
    SetI2cBaudRate = 0x10,
    SwitchKeymap = 0x11,
    GetVariable = 0x12,
    SetVariable = 0x13,
    ExecMacroCommand = 0x14,
}

pub enum EepromOperation {
    Read = 0,
    Write = 1,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Copy, Clone)]
#[repr(u8)]
pub enum ConfigBufferId {
    HardwareConfig = 0,
    StagingUserConfig = 1,
    ValidatedUserConfig = 2,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum DevicePropertyIds {
    DeviceProtocolVersion = 0,
    ProtocolVersions = 1,
    ConfigSizes = 2,
    CurrentKbootCommand = 3,
    I2cBaudRate = 4,
    Uptime = 5,
    GitTag = 6,
    GitRepo = 7,
}

pub enum EnumerationModes {
    Bootloader = 0,
    Buspal = 1,
    NormalKeyboard = 2,
    CompatibleKeyboard = 3,
}

pub enum KbootCommands {
    Idle = 0,
    Ping = 1,
    Reset = 2,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum ModulePropertyId {
    ProtocolVersions = 0,
    GitTag = 1,
    GitRepo = 2,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum UsbVariables {
    TestSwitches = 0,
    TestUsbStack = 1,
    DebounceTimePress = 2,
    DebounceTimeRelease = 3,
    UsbReportSemaphore = 4,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum MacroActionId {
    KeyMacroAction = 0,
    /*
        0 - 63 are reserved for KeyMacroAction
        2 bits for: PressKeyMacroAction / HoldKeyMacroAction / ReleaseKeyMacroAction / undefined
        2 bits for: with only scancode / only modifiers / both scancode and modifiers / undefined
        2 bits for: scancode type basic, short media, long media, system. It should be only used if scancode does exist.
    */
    LastKeyMacroAction = 63,
    MouseButtonMacroAction = 64,
    /*
        64 - 66 are reserved for MouseButtonMacroAction
        PressMouseButtonsMacroAction    = 64,
        HoldMouseButtonsMacroAction     = 65,
        ReleaseMouseButtonsMacroAction  = 66,
    */
    LastMouseButtonMacroAction = 66,
    MoveMouseMacroAction = 67,
    ScrollMouseMacroAction = 68,
    DelayMacroAction = 69,
    TextMacroAction = 70,
    CommandMacroAction = 71,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum KeyActionId {
    NoneAction = 0,
    KeystrokeAction = 1,
    /*
        1 - 31 are reserved for KeystrokeAction
        5 bits:
            1: Do we have scancode?
            2: Do we have modifiers?
            3: Do we have secondaryRole?
            4-5: What kind of keystroke? (basic, short/long media, system)
    */
    LastKeystrokeAction = 31,
    SwitchLayerAction = 32,
    SwitchKeymapAction = 33,
    MouseAction = 34,
    PlayMacroAction = 35,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum KeystrokeActionFlag {
    Scancode = 1 << 0,
    ModifierMask = 1 << 1,
    SecondaryRoleAction = 1 << 2,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum KeystrokeType {
    Basic = 0,
    ShortMedia = 1,
    LongMedia = 2,
    System = 3,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum SwitchLayerMode {
    HoldAndDoubleTapToggle = 0,
    Toggle = 1,
    Hold = 2,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum MouseActionParam {
    LeftClick = 0,
    MiddleClick,
    RightClick,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    Accelerate,
    Decelerate,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}

pub const LAYER_NUMBER_TO_STRING: [&str; 4] = ["base", "mod", "fn", "mouse"];
