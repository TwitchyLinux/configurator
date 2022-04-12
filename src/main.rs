use druid::{AppLauncher, PlatformError, WidgetExt, WindowDesc};
use structopt::StructOpt;

use configurator::{Cmd, EscExiter, Opt};

fn main() -> Result<(), PlatformError> {
    let mut args = Opt::from_args();
    if args.file.starts_with("~") {
        args.file = args
            .file
            .to_str()
            .unwrap()
            .replace("~", home::home_dir().unwrap().to_str().unwrap())
            .into();
    }

    match &args.cmd {
        Cmd::Display => {
            use configurator::{display::build_ui, model::display::App};

            let mut conn = swayipc::Connection::new().unwrap();
            let model: App = conn.get_outputs().unwrap().into();

            return AppLauncher::with_window(
                WindowDesc::new(build_ui(&args).controller(EscExiter {}))
                    .title("TwitchyLinux - Configure display")
                    .window_size((600.0, 700.0)),
            )
            .launch(model);
        }

        Cmd::Bluetooth => {
            use configurator::{
                bluetooth::{build_ui, BluetoothDelegate},
                model::bluetooth::App,
            };
            let model: App = App::default();

            let launcher = AppLauncher::with_window(
                WindowDesc::new(build_ui(&args).controller(EscExiter {}))
                    .title("TwitchyLinux - Configure bluetooth")
                    .window_size((700.0, 600.0)),
            );

            let sink = launcher.get_external_handle();
            return launcher
                .delegate(BluetoothDelegate::new(sink))
                .launch(model);
        }
    }

    Ok(())
}
