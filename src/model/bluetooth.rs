use druid::im::{HashMap, Vector};
use druid::{Data, Lens};

#[derive(Clone, Default, Data, Lens)]
pub struct App {
    pub scanning: bool,
    pub status_text: String,
}
