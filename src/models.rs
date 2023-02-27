pub const UHK_VENDOR_ID: u16 = 0x1D50;

pub struct UhkDeviceProduct {
    pub vendor_id: u16,
    pub keyboard_pid: u16,
    pub bootloader_pid: u16,
    pub buspal_pid: u16,
}

pub const UHK_60_V2_DEVICE: UhkDeviceProduct = UhkDeviceProduct {
    vendor_id: UHK_VENDOR_ID,
    keyboard_pid: 0x6124,
    bootloader_pid: 0x6123,
    buspal_pid: 0x6121,
};
