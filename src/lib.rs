pub mod display;
pub mod lens;
pub mod model;
pub mod widgets;

use std::path::PathBuf;
use structopt::StructOpt;

#[allow(non_camel_case_types)]
#[derive(StructOpt, Debug, PartialEq, Clone)]
pub enum Cmd {
    /// Open a UI to configure the displays.
    Display,
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "twl-configurator",
    about = "Graphical frontend for configuring Sway on TwitchyLinux"
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Cmd,
    #[structopt(
        short = "c",
        name = "sway config base directory",
        long = "config_base",
        default_value = "~/.config/sway/twl"
    )]
    pub file: PathBuf,
}
