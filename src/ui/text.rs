use super::{Image, Ui, Update};
use crate::UiManager;
use ab_glyph::{point, Font, FontRef};
use hex::{
    anyhow,
    ecs::{ev::Control, Ev},
    glium::glutin::event::Event,
};
use std::mem;

pub struct Text<'a> {
    pub content: String,
    pub font: FontRef<'a>,
    pub scale: f32,
    pub active: bool,
}

impl Ui for Text<'_> {
    fn ui<'a>(&mut self, _: &mut Ev, _: &mut UiManager) -> anyhow::Result<Option<Update<'a>>> {
        if self.active {
            Ok(Some(Update::new(move |e, event, world| {
                if let Ev::Event(Control {
                    event: Event::MainEventsCleared,
                    flow: _,
                }) = event
                {
                    if let Some((_i, t)) = world.cm.get::<Image>(e, &world.em).and_then(|i| {
                        Some((
                            i,
                            world
                                .cm
                                .get::<Box<dyn Ui>>(e, &world.em)
                                .map(|u| unsafe { mem::transmute::<_, Box<Self>>(u) })?,
                        ))
                    }) {
                        for g in t.content.chars().map(|c| {
                            t.font
                                .glyph_id(c)
                                .with_scale_and_position(t.scale, point(100.0, 0.0))
                        }) {
                            if let Some(q) = t.font.outline_glyph(g) {
                                q.draw(|_x, _y, _c| todo!());
                            }
                        }
                    }
                }

                Ok(())
            })))
        } else {
            Ok(None)
        }
    }
}
