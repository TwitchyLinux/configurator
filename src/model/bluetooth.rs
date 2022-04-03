use druid::im::{HashMap, Vector};
use druid::{Data, Lens};

pub mod commands {
    use druid::Selector;

    // UI -> worker commands
    pub const CONNECT_TO_DEVICE: Selector<u32> = Selector::new("connect_to_device");

    // worker -> UI commands
    pub const UPDATE_STATUS: Selector<String> = Selector::new("update_status");
}

#[derive(Clone, Default, Data, Lens)]
pub struct App {
    pub scanning: bool,
    pub status_text: String,
}
