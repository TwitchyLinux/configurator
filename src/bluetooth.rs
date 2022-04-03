use druid::text::ParseFormatter;
use druid::{
    AppDelegate, Color, Command, DelegateCtx, ExtEventSink, Handled, Lens, LensExt, Selector,
    Target, Widget, WidgetExt,
};

use crate::model::bluetooth::{commands, App};

use crate::Opt;
use druid::widget::prelude::*;
use druid::widget::{
    Button, Controller, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Painter,
    RadioGroup, Scroll, SizedBox, Split, Switch, TextBox,
};

use bluez::management::client::*;
use futures::executor::block_on;
use std::sync::mpsc;
use std::thread;

pub struct BluetoothDelegate {
    thread: thread::JoinHandle<()>,
    tx: mpsc::Sender<u32>,
}

impl BluetoothDelegate {
    pub fn new(sink: ExtEventSink) -> Self {
        let (tx, rx) = mpsc::channel();

        BluetoothDelegate {
            thread: thread::spawn(move || {
                let rx: mpsc::Receiver<u32> = rx;
                let sink: ExtEventSink = sink;

                let mut client = match ManagementClient::new() {
                    Ok(client) => client,
                    Err(e) => {
                        sink.submit_command(
                            commands::UPDATE_STATUS,
                            String::from(format!("Error 1: {:?}", e)),
                            Target::Auto,
                        );
                        return;
                    }
                };

                let controllers = match block_on(client.get_controller_list()) {
                    Ok(controllers) => controllers,
                    Err(e) => {
                        sink.submit_command(
                            commands::UPDATE_STATUS,
                            String::from(format!("Error 2: {:?}", e)),
                            Target::Auto,
                        );
                        return;
                    }
                };
                if controllers.len() == 0 {
                    sink.submit_command(
                        commands::UPDATE_STATUS,
                        "No bluetooth controllers found!".to_string(),
                        Target::Auto,
                    );
                    return;
                }

                println!("controllers: {:?}", controllers);
            }),
            tx,
        }
    }
}

impl AppDelegate<App> for BluetoothDelegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut App,
        _env: &Env,
    ) -> Handled {
        // UI -> worker
        if let Some(number) = cmd.get(commands::CONNECT_TO_DEVICE) {
            // TODO: stuff
            return Handled::Yes;
        }

        // worker -> UI
        if let Some(msg) = cmd.get(commands::UPDATE_STATUS) {
            data.status_text = msg.clone();
            return Handled::Yes;
        }

        Handled::No
    }
}

fn build_topbar() -> impl Widget<App> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::End)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(
            Label::new(|data: &App, _env: &_| data.status_text.clone()).expand_width(),
            0.5,
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(Switch::new().lens(App::scanning))
                .with_default_spacer()
                .with_child(Label::new("Scanning")),
        )
}

fn build_buttons(args: &Opt) -> impl Widget<App> {
    let base_path = args.file.clone();

    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::End)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_child(
            Button::new("Apply now")
                .on_click(|_ctx, _data: &mut App, _env| println!("apply clicked")),
        )
        .with_default_spacer()
        .with_child(
            Button::new("Save config")
                .on_click(move |_ctx, data: &mut App, _env| println!("save clicked")),
        )
        .with_default_spacer()
        .expand_width()
}

pub fn build_ui(args: &Opt) -> impl Widget<App> {
    Flex::column()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_child(build_topbar())
        .with_default_spacer()
        .with_flex_child(
            Flex::column()
                .must_fill_main_axis(true)
                .main_axis_alignment(MainAxisAlignment::Start)
                .cross_axis_alignment(CrossAxisAlignment::Center),
            0.9,
        )
        .with_default_spacer()
        .with_child(build_buttons(args))
    // .debug_paint_layout()
}
