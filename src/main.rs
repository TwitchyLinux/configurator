use druid::text::ParseFormatter;
use druid::{AppLauncher, Color, Lens, LensExt, PlatformError, Widget, WidgetExt, WindowDesc};

use configurator::{AppData, DisplayInfo, FocusedDisplay, Pos, Scale, Transform};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, RadioGroup, Scroll, SizedBox,
    Split, TextBox,
};
use std::path::PathBuf;
use structopt::StructOpt;

fn build_name_row() -> impl Widget<AppData> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Name")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(
                Label::new(|d: &AppData, _: &druid::Env| {
                    FocusedDisplay.with(d, |d| {
                        d.as_ref()
                            .map_or("<no selection>".to_string(), |d| d.name.clone())
                    })
                })
                .with_text_color(Color::rgb8(200, 200, 200)),
            )
            .expand_width(),
            0.7,
        )
}

fn build_serial_row() -> impl Widget<AppData> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Serial")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(
                Label::new(|d: &AppData, _: &druid::Env| {
                    FocusedDisplay.with(d, |d| {
                        d.as_ref()
                            .map_or("<no selection>".to_string(), |d| d.serial.clone())
                    })
                })
                .with_text_color(Color::rgb8(200, 200, 200)),
            )
            .expand_width(),
            0.7,
        )
}

fn build_make_row() -> impl Widget<AppData> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Make")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(
                Label::new(|d: &AppData, _: &druid::Env| {
                    FocusedDisplay.with(d, |d| {
                        d.as_ref()
                            .map_or("<no selection>".to_string(), |d| d.make.clone())
                    })
                })
                .with_text_color(Color::rgb8(200, 200, 200)),
            )
            .expand_width(),
            0.7,
        )
}

fn build_model_row() -> impl Widget<AppData> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Model")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(
                Label::new(|d: &AppData, _: &druid::Env| {
                    FocusedDisplay.with(d, |d| {
                        d.as_ref()
                            .map_or("<no selection>".to_string(), |d| d.model.clone())
                    })
                })
                .with_text_color(Color::rgb8(200, 200, 200)),
            )
            .expand_width(),
            0.7,
        )
}

fn build_pos_input() -> impl Widget<AppData> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Position")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(
                TextBox::new()
                    .with_formatter(ParseFormatter::<Pos>::new())
                    .update_data_while_editing(true)
                    .lens(
                        FocusedDisplay
                            .map(
                                |x| x.as_ref().unwrap_or(&DisplayInfo::default()).clone(),
                                |x, y| {
                                    if x.is_some() {
                                        *x = Some(y)
                                    }
                                },
                            )
                            .then(DisplayInfo::position),
                    ),
            )
            .expand_width(),
            0.7,
        )
}

fn build_scale_input() -> impl Widget<AppData> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Scale")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(
                TextBox::new()
                    .with_formatter(ParseFormatter::<Scale>::new())
                    .update_data_while_editing(true)
                    .lens(
                        FocusedDisplay
                            .map(
                                |x| x.as_ref().unwrap_or(&DisplayInfo::default()).clone(),
                                |x, y| {
                                    if x.is_some() {
                                        *x = Some(y)
                                    }
                                },
                            )
                            .then(DisplayInfo::scale),
                    ),
            )
            .expand_width(),
            0.7,
        )
}

fn build_rotation_input() -> impl Widget<AppData> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Rotation")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                SizedBox::new(
                    RadioGroup::new(vec![
                        ("None", Transform::None),
                        ("90", Transform::R90),
                        ("180", Transform::R180),
                        ("270", Transform::R270),
                    ])
                    .lens(
                        FocusedDisplay
                            .map(
                                |x| x.as_ref().unwrap_or(&DisplayInfo::default()).clone(),
                                |x, y| {
                                    if x.is_some() {
                                        *x = Some(y)
                                    }
                                },
                            )
                            .then(DisplayInfo::transform),
                    ),
                )
                .expand_width(),
            )
            .vertical(),
            0.7,
        )
}

fn build_buttons(args: &Opt) -> impl Widget<AppData> {
    let base_path = args.file.clone();

    SizedBox::new(
        Flex::row()
            .must_fill_main_axis(true)
            .main_axis_alignment(MainAxisAlignment::End)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .with_default_spacer()
            .with_flex_child(
                Button::new("Apply now")
                    .on_click(|_ctx, data: &mut AppData, _env| data.apply_displays()),
                0.3,
            )
            .with_default_spacer()
            .with_flex_child(
                Button::new("Save config").on_click(move |_ctx, data: &mut AppData, _env| {
                    data.save_config(base_path.clone()).unwrap()
                }),
                0.3,
            )
            .with_default_spacer(),
    )
    .expand_width()
}

fn build_ui(args: &Opt) -> impl Widget<AppData> {
    Split::rows(
        configurator::MonitorView::default().lens(AppData::display_geo),
        Flex::column()
            .must_fill_main_axis(true)
            .main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .with_flex_child(
                Flex::column()
                    .must_fill_main_axis(true)
                    .main_axis_alignment(MainAxisAlignment::Start)
                    .cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_flex_child(build_name_row(), 0.1)
                    .with_flex_child(build_serial_row(), 0.1)
                    .with_flex_child(build_make_row(), 0.1)
                    .with_flex_child(build_model_row(), 0.1)
                    .with_default_spacer()
                    .with_flex_child(build_pos_input(), 0.1)
                    .with_flex_child(build_scale_input(), 0.1)
                    .with_default_spacer()
                    .with_flex_child(build_rotation_input(), 0.2),
                0.9,
            )
            .with_default_spacer()
            .with_flex_child(build_buttons(args), 0.2),
    )
    .bar_size(2.)
    .solid_bar(true)
    .draggable(true)
    //.debug_paint_layout()
}

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
struct Opt {
    #[structopt(subcommand)]
    cmd: Cmd,
    #[structopt(
        short = "c",
        name = "sway config base directory",
        long = "config_base",
        default_value = "~/.config/sway/twl"
    )]
    file: PathBuf,
}

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
            let mut conn = swayipc::Connection::new().unwrap();
            let model: AppData = conn.get_outputs().unwrap().into();

            return AppLauncher::with_window(
                WindowDesc::new(build_ui(&args))
                    .title("TwitchyLinux - Configure display")
                    .window_size((600.0, 600.0)),
            )
            .launch(model);
        }
    }

    Ok(())
}