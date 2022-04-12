use druid::im::{HashMap, Vector};
use druid::{Data, Lens};

pub mod commands {
    use druid::im::Vector;
    use druid::Selector;

    // UI -> worker commands
    pub const CONNECT_TO_DEVICE: Selector<u32> = Selector::new("connect_to_device");
    pub const DO_SCAN: Selector<bool> = Selector::new("do_scan");
    pub const ENUM_DEVICES: Selector<()> = Selector::new("enumerate_devices");

    // worker -> UI commands
    pub const UPDATE_STATUS: Selector<String> = Selector::new("update_status");
    pub const UPDATE_DEVICES: Selector<Vector<super::Device>> = Selector::new("update_devices");
}

#[derive(Clone, Default, Data, Lens)]
pub struct App {
    pub scanning: bool,
    pub status_text: String,

    pub devices: Vector<Device>,
}

#[derive(Clone, Default, Data, Debug, Lens)]
pub struct Device {
    pub id: String,
    pub addr: String,
    pub name: String,

    pub rssi: Option<i16>,
}
