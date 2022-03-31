use druid::text::ParseFormatter;
use druid::{Color, Lens, LensExt, Widget, WidgetExt};

use crate::lens::FocusedDisplay;
use crate::model::display::{App, Display, Mode, Pos, Scale, Transform};
use crate::widgets::display::MonitorView;
use crate::Opt;
use druid::widget::prelude::*;
use druid::widget::{
    Button, Controller, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Painter,
    RadioGroup, Scroll, SizedBox, Split, TextBox,
};

fn build_name_row() -> impl Widget<App> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Name")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(
                Label::new(|d: &App, _: &druid::Env| {
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

fn build_info_row() -> impl Widget<App> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(Label::new("Model information")).expand_width(),
            0.3,
        )
        .with_default_spacer()
        .with_flex_child(
            SizedBox::new(
                Label::new(|d: &App, _: &druid::Env| {
                    FocusedDisplay.with(d, |d| {
                        d.as_ref().map_or("<no selection>".to_string(), |d| {
                            let mut o = d.make.clone();
                            o.push_str(", ");
                            o.push_str(&d.model);
                            o.push_str(" (");
                            o.push_str(&d.serial);
                            o.push(')');
                            o
                        })
                    })
                })
                .with_text_color(Color::rgb8(200, 200, 200)),
            )
            .expand_width(),
            0.7,
        )
}

fn build_pos_input() -> impl Widget<App> {
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
                                |x| x.as_ref().unwrap_or(&Display::default()).clone(),
                                |x, y| {
                                    if x.is_some() {
                                        *x = Some(y)
                                    }
                                },
                            )
                            .then(Display::position),
                    ),
            )
            .expand_width(),
            0.7,
        )
        .with_spacer(2.)
}

fn build_scale_input() -> impl Widget<App> {
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
                                |x| x.as_ref().unwrap_or(&Display::default()).clone(),
                                |x, y| {
                                    if x.is_some() {
                                        *x = Some(y)
                                    }
                                },
                            )
                            .then(Display::scale),
                    ),
            )
            .expand_width(),
            0.7,
        )
        .with_spacer(2.)
}

fn build_rotation_input() -> impl Widget<App> {
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
                    RadioGroup::row(vec![
                        ("None", Transform::None),
                        ("90", Transform::R90),
                        ("180", Transform::R180),
                        ("270", Transform::R270),
                    ])
                    .lens(
                        FocusedDisplay
                            .map(
                                |x| x.as_ref().unwrap_or(&Display::default()).clone(),
                                |x, y| {
                                    if x.is_some() {
                                        *x = Some(y)
                                    }
                                },
                            )
                            .then(Display::transform),
                    ),
                )
                .expand_width(),
            )
            .horizontal(),
            0.7,
        )
}

const MODE_SELECTED_ACTION: druid::Selector<Mode> = druid::Selector::new("mode_selected_action");

struct SingleModeController;

impl<W: Widget<App>> Controller<App, W> for SingleModeController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut App,
        env: &Env,
    ) {
        if let Event::Command(c) = event {
            if let Some(nm) = c.get(MODE_SELECTED_ACTION) {
                for (_, d) in data.display_geo.iter_mut() {
                    if d.focused {
                        for m in d.modes.iter_mut() {
                            m.selected = m.width == nm.width
                                && m.height == nm.height
                                && m.refresh == nm.refresh;
                        }
                    }
                }
                ctx.set_handled();
            }
        }

        child.event(ctx, event, data, env)
    }
}

fn build_mode_input() -> impl Widget<App> {
    Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(SizedBox::new(Label::new("Mode")).expand_width(), 0.3)
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    let painter = Painter::new(|ctx, data: &Mode, env| {
                        let bounds = ctx.size().to_rect();
                        ctx.fill(
                            bounds,
                            &env.get(if data.selected {
                                druid::theme::PRIMARY_DARK
                            } else {
                                druid::theme::WINDOW_BACKGROUND_COLOR
                            }),
                        );
                    });

                    SizedBox::new(
                        Label::new(|item: &Mode, _env: &_| {
                            format!(
                                "{}x{}, {:.2} Hz",
                                item.width,
                                item.height,
                                item.refresh as f64 / 1000.
                            )
                        })
                        .align_vertical(druid::UnitPoint::LEFT)
                        .background(painter)
                        .on_click(move |ctx, data: &mut Mode, _env| {
                            ctx.submit_command(druid::Command::new(
                                MODE_SELECTED_ACTION,
                                data.clone(),
                                druid::Target::Auto,
                            ))
                        }),
                    )
                    .expand_width()
                })
                .lens(
                    FocusedDisplay
                        .map(
                            |x| x.as_ref().unwrap_or(&Display::default()).clone(),
                            |x, y| {
                                if x.is_some() {
                                    *x = Some(y)
                                }
                            },
                        )
                        .then(Display::modes),
                )
                .controller(SingleModeController {}),
            )
            .vertical()
            .fix_height(120.),
            0.7,
        )
}

fn build_buttons(args: &Opt) -> impl Widget<App> {
    let base_path = args.file.clone();

    SizedBox::new(
        Flex::row()
            .must_fill_main_axis(true)
            .main_axis_alignment(MainAxisAlignment::End)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .with_default_spacer()
            .with_flex_child(
                Button::new("Apply now")
                    .on_click(|_ctx, data: &mut App, _env| data.apply_displays()),
                0.3,
            )
            .with_default_spacer()
            .with_flex_child(
                Button::new("Save config").on_click(move |_ctx, data: &mut App, _env| {
                    data.save_config(base_path.clone()).unwrap()
                }),
                0.3,
            )
            .with_default_spacer(),
    )
    .expand_width()
}

pub fn build_ui(args: &Opt) -> impl Widget<App> {
    Split::rows(
        MonitorView::default().lens(App::display_geo),
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
                    .with_flex_child(build_info_row(), 0.1)
                    .with_default_spacer()
                    .with_flex_child(build_pos_input(), 0.1)
                    .with_spacer(2.)
                    .with_flex_child(build_scale_input(), 0.1)
                    .with_default_spacer()
                    .with_flex_child(build_rotation_input(), 0.2)
                    .with_default_spacer()
                    .with_flex_child(build_mode_input(), 0.5),
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
