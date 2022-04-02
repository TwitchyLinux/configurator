use druid::text::ParseFormatter;
use druid::{Color, Lens, LensExt, Widget, WidgetExt};

use crate::model::bluetooth::App;

use crate::Opt;
use druid::widget::prelude::*;
use druid::widget::{
    Button, Controller, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Painter,
    RadioGroup, Scroll, SizedBox, Split, Switch, TextBox,
};

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
