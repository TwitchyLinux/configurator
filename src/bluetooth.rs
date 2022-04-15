use druid::{
    AppDelegate, Color, Command, DelegateCtx, ExtEventSink, Handled, Lens, LensExt, Selector,
    Target, Widget, WidgetExt,
};

use crate::model::bluetooth::{commands, App, Device};

use crate::Opt;
use druid::widget::prelude::*;
use druid::widget::{
    Button, Controller, CrossAxisAlignment, Either, Flex, FlexParams, Label, List,
    MainAxisAlignment, Painter, RadioGroup, Scroll, SizedBox, Split, Switch, TextBox,
};

use blurz::{
    bluetooth_event::BluetoothEvent as BtEvent, BluetoothAdapter, BluetoothDevice,
    BluetoothDiscoverySession, BluetoothSession,
};
use futures::executor::block_on;
use std::sync::mpsc;
use std::thread;

pub struct BluetoothDelegate<'a> {
    session: &'static mut BluetoothSession,
    adapter: BluetoothAdapter<'a>,
    bds: BluetoothDiscoverySession<'a>,
    sink: ExtEventSink,
}

impl<'a> BluetoothDelegate<'a> {
    pub fn new(sink: ExtEventSink) -> Self {
        let adapter = BluetoothAdapter::init(Box::leak(Box::new(
            BluetoothSession::create_session(None).unwrap(),
        )))
        .unwrap();

        sink.submit_command(
            commands::UPDATE_STATUS,
            format!(
                "Using: {} ({})",
                adapter.get_id(),
                adapter.get_name().unwrap()
            ),
            Target::Auto,
        );

        if !adapter.is_powered().unwrap() {
            adapter.set_powered(true).unwrap();
        }

        let bds = BluetoothDiscoverySession::create_session(
            Box::leak(Box::new(BluetoothSession::create_session(None).unwrap())),
            adapter.get_id(),
        )
        .unwrap();

        let session: &'static mut BluetoothSession =
            Box::leak(Box::new(BluetoothSession::create_session(None).unwrap()));

        BluetoothDelegate {
            session,
            adapter,
            bds,
            sink,
        }
    }
}

impl AppDelegate<App> for BluetoothDelegate<'_> {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut App,
        _env: &Env,
    ) -> Handled {
        // UI -> worker
        if let Some(id) = cmd.get(commands::CONNECT_TO_DEVICE) {
            let dev = BluetoothDevice::new(self.session, id.clone());
            match dev.connect(6500) {
                Ok(()) => {}
                Err(e) => {
                    println!("connecting to {} failed: {:?}", id, e)
                }
            }
            return Handled::Yes;
        }
        if let Some(id) = cmd.get(commands::DISCONNECT_FROM_DEVICE) {
            let dev = BluetoothDevice::new(self.session, id.clone());
            match dev.disconnect() {
                Ok(()) => {}
                Err(e) => {
                    println!("disconnecting from {} failed: {:?}", id, e)
                }
            }
            return Handled::Yes;
        }
        if let Some(want) = cmd.get(commands::DO_SCAN) {
            println!("want scanning state {}", want);
            match self.adapter.set_discoverable(*want) {
                Ok(devices) => {
                    data.scanning = *want;
                }
                Err(e) => println!("set_discoverable error: {:?}", e),
            };
            if *want {
                self.bds.start_discovery().unwrap();
            } else {
                self.bds.stop_discovery().unwrap();
            }
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(commands::ENUM_DEVICES) {
            match self.adapter.get_device_list() {
                Ok(devices) => {
                    data.devices = devices
                        .into_iter()
                        .map(|d| {
                            let device = BluetoothDevice::new(self.session, d);

                            Device {
                                id: device.get_id(),
                                addr: device.get_address().unwrap(),
                                name: device.get_name().unwrap_or("".into()),
                                rssi: device.get_rssi().ok(),
                                connected: device.is_connected().unwrap(),
                            }
                        })
                        .collect();
                }
                Err(e) => println!("Enum error: {:?}", e),
            };

            data.devices.sort_by(|a, b| match (a.rssi, b.rssi) {
                (Some(a), Some(b)) => b.cmp(&a),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            });

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

struct PollController {
    timer: druid::TimerToken,
}

impl Default for PollController {
    fn default() -> Self {
        Self {
            timer: druid::TimerToken::INVALID,
        }
    }
}

impl<W: Widget<App>> Controller<App, W> for PollController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut App,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                self.timer = ctx.request_timer(std::time::Duration::from_millis(160));
            }
            Event::Timer(id) => {
                if *id == self.timer {
                    self.timer = ctx.request_timer(std::time::Duration::from_millis(1250));
                    ctx.submit_command(commands::ENUM_DEVICES);
                }
            }
            _ => {}
        }

        child.event(ctx, event, data, env)
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
                .with_child(Switch::new().lens(App::scanning).on_click(
                    |ctx, data: &mut App, _env| {
                        ctx.submit_command(Command::new(
                            commands::DO_SCAN,
                            !data.scanning,
                            Target::Auto,
                        ));
                    },
                ))
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

fn build_device_entry() -> impl Widget<Device> {
    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(
            Flex::row()
                .must_fill_main_axis(true)
                .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Label::new(|item: &Device, _env: &_| {
                        if &item.name == "" {
                            "unnamed device".to_string()
                        } else {
                            item.name.clone()
                        }
                    })
                    .fix_width(205.),
                )
                .with_flex_child(
                    Label::new(|item: &Device, _env: &_| {
                        if let Some(rssi) = &item.rssi {
                            format!("{}", rssi)
                        } else {
                            "   ".to_string()
                        }
                    }),
                    FlexParams::new(0.1, CrossAxisAlignment::Center),
                )
                //.with_flex_child(Label::new(""), 0.8)
                .with_flex_child(
                    Label::new(|item: &Device, _env: &_| item.addr.clone()),
                    FlexParams::new(0.1, CrossAxisAlignment::End),
                )
                .with_flex_child(
                    Either::new(
                        |item: &Device, _env: &_| item.connected,
                        Button::new("Disconnect").on_click(move |ctx, data: &mut Device, _env| {
                            ctx.submit_command(
                                commands::DISCONNECT_FROM_DEVICE.with(data.id.clone()),
                            )
                        }),
                        Button::new("Connect").on_click(move |ctx, data: &mut Device, _env| {
                            ctx.submit_command(commands::CONNECT_TO_DEVICE.with(data.id.clone()))
                        }),
                    ),
                    FlexParams::new(0.2, CrossAxisAlignment::End),
                )
                .with_spacer(0.05),
        )
    //.debug_paint_layout()
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
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .with_flex_child(
                    Scroll::new(List::new(build_device_entry).lens(App::devices)).vertical(),
                    0.9,
                )
                .padding(12.),
            0.9,
        )
        .with_default_spacer()
        .with_child(build_buttons(args))
        .controller(PollController::default())
    // .debug_paint_layout()
}
