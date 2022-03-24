use druid::piet::{FontFamily, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{Affine, Color};

use crate::DisplayInfo;
use druid::im::HashMap;

#[derive(Clone, Default, Debug)]
struct DragState {
    start: (f64, f64),
    target: Option<(String, (i32, i32))>,
}

pub struct MonitorView {
    pub offset: (f64, f64),
    pub scale: f64,

    dragging: Option<DragState>,
}

impl Default for MonitorView {
    fn default() -> MonitorView {
        MonitorView {
            offset: (0., 0.),
            scale: 1.,
            dragging: None,
        }
    }
}

impl MonitorView {
    fn window_space_bounds(&self, data: &HashMap<String, DisplayInfo>) -> ((f64, f64), (f64, f64)) {
        data.iter().fold(
            ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN)),
            |acc, (_, x)| {
                (
                    (
                        acc.0 .0.min(x.position.0 as f64),
                        acc.0 .1.min(x.position.1 as f64),
                    ),
                    (
                        acc.1 .0.max(x.position.0 as f64 + x.size.0 as f64),
                        acc.1 .1.max(x.position.1 as f64 + x.size.1 as f64),
                    ),
                )
            },
        )
    }

    fn center_displays(&mut self, data: &HashMap<String, DisplayInfo>, bounds: Size) {
        let (min, max) = self.window_space_bounds(data);
        let (width, height) = (max.0 - min.0, max.1 - min.1);
        let min_box = (bounds - Size::new(40., 50.))
            .to_rect()
            .contained_rect_with_aspect_ratio(height / width);
        let scale = (min_box.width() / width)
            .min(min_box.height() / height)
            .min(1.);

        // println!("max rect = {:?}", min_box);
        // println!("w = {:?}\nh = {:?}", width, height);
        // println!("scale = {:?}", scale);
        self.scale = scale;

        self.offset = (
            (bounds.width - min_box.width()) / 2.,
            (bounds.height - min_box.height()) / 2.,
        );
    }

    fn unscale_coords(&self, p: (f64, f64)) -> (f64, f64) {
        (
            (p.0 - self.offset.0) / self.scale,
            (p.1 - self.offset.1) / self.scale,
        )
    }

    fn hit_test(&self, data: &HashMap<String, DisplayInfo>, pos: druid::Point) -> Option<String> {
        let coords = self.unscale_coords(pos.into());

        for (_, d) in data {
            let bb = druid::Rect::new(
                d.position.0 as f64,
                d.position.1 as f64,
                d.position.0 as f64 + d.size.0 as f64,
                d.position.1 as f64 + d.size.1 as f64,
            );
            if bb.contains(coords.into()) {
                return Some(d.name.clone());
            }
        }

        None
    }

    fn update_focus(&self, data: &mut HashMap<String, DisplayInfo>, name: &String) {
        for (n, d) in data.iter_mut() {
            d.focused = n == name;
        }
    }

    fn normalize_coords(&self, data: &mut HashMap<String, DisplayInfo>) {
        let (min, _) = self.window_space_bounds(data);

        for (_, d) in data.iter_mut() {
            d.position = (d.position.0 - min.0 as i32, d.position.1 - min.1 as i32).into();
        }
    }

    fn quantize(&self, pos: (i32, i32)) -> (i32, i32) {
        const QUANT: i32 = 16;

        (
            ((pos.0 / QUANT) * QUANT) + (QUANT / 2),
            ((pos.1 / QUANT) * QUANT) + (QUANT / 2),
        )
    }
}

impl Widget<HashMap<String, DisplayInfo>> for MonitorView {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut HashMap<String, DisplayInfo>,
        _env: &Env,
    ) {
        match event {
            Event::MouseDown(e) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_focus();
                    let target = self.hit_test(data, e.pos);

                    match &self.dragging {
                        None => {
                            match target {
                                Some(n) => {
                                    let pos = data.get(&n).unwrap().position;
                                    self.update_focus(data, &n);
                                    self.dragging = Some(DragState {
                                        start: e.pos.into(),
                                        target: Some((n, pos.into())),
                                    });
                                }
                                None => {
                                    self.dragging = Some(DragState {
                                        start: (e.pos - self.offset).into(),
                                        target: None,
                                    });
                                    self.update_focus(data, &"".to_string())
                                }
                            };
                        }
                        Some(d) => unreachable!("already dragging {:?}", d),
                    }

                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    match &self.dragging {
                        Some(drag) => {
                            if drag.target.is_some() {
                                self.normalize_coords(data);
                            }
                            self.dragging = None;
                            ctx.request_layout();
                        }
                        None => {}
                    }
                }
                ctx.set_active(false);
            }
            Event::MouseMove(e) => match &self.dragging {
                Some(drag) => {
                    if let Some((target, start_pos)) = &drag.target {
                        let (sp_x, sp_y) = (start_pos.0, start_pos.1);

                        data.get_mut(target).unwrap().position = self
                            .quantize((
                                (sp_x as f64 - (drag.start.0 as f64 - e.pos.x) / self.scale) as i32,
                                (sp_y as f64 - (drag.start.1 as f64 - e.pos.y) / self.scale) as i32,
                            ))
                            .into();
                    } else {
                        self.offset = (-(drag.start.0 - e.pos.x), -(drag.start.1 - e.pos.y));
                    }
                    ctx.request_paint();
                }
                None => {}
            },
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &HashMap<String, DisplayInfo>,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &HashMap<String, DisplayInfo>,
        _data: &HashMap<String, DisplayInfo>,
        _env: &Env,
    ) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &HashMap<String, DisplayInfo>,
        _env: &Env,
    ) -> Size {
        let size = bc.constrain(Size::new(600.0, 500.0));
        self.center_displays(data, size);

        size
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &HashMap<String, DisplayInfo>, env: &Env) {
        // Clear the whole widget
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &env.get(druid::theme::WINDOW_BACKGROUND_COLOR));

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(self.offset));
            ctx.transform(Affine::scale(self.scale));

            for (_id, d) in data.iter() {
                let (x1, y1) = (d.position.0 as f64, d.position.1 as f64);
                let (x2, y2) = (x1 + d.size.0 as f64, y1 + d.size.1 as f64);
                let id = d.id.clone().unwrap_or(12);
                let name = d.name.clone();
                let scale = self.scale;
                let focused = d.focused;

                let fill_color = env.get(druid::theme::BACKGROUND_DARK);
                let stroke_color = if focused {
                    env.get(druid::theme::TEXT_COLOR)
                } else {
                    env.get(druid::theme::PRIMARY_DARK)
                };

                ctx.paint_with_z_index(id as u32, move |ctx| {
                    let rect =
                        druid::Rect::new(x1 + 8., y1 + 8., x2 - 8., y2 - 8.).to_rounded_rect(25.);
                    ctx.fill(rect, &fill_color);
                    ctx.stroke(rect, &stroke_color, if focused { 10.0 } else { 5.0 });

                    let text = ctx.text();
                    let layout = text
                        .new_text_layout(name)
                        .font(FontFamily::SERIF, 18.0 / scale)
                        .text_color(Color::rgb8(255, 255, 255))
                        .build()
                        .unwrap();

                    ctx.clip(rect);
                    ctx.draw_text(&layout, (x1 + 35.0, y1 + 35.0));
                });
            }
        });
    }
}
