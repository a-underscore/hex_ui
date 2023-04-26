use super::{Ui, Update};
use crate::{ScreenPos, UiCallback, UiManager};
use hex::{
    anyhow,
    components::Camera,
    ecs::{ev::Control, Ev},
    glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent},
    math::{Mat3d, Vec2d},
};

#[derive(Clone)]
pub struct Button {
    pub dimensions: Vec2d,
    pub active: bool,
}

impl Ui for Button {
    fn ui<'a>(
        &mut self,
        _: &mut Ev,
        manager: &mut UiManager,
    ) -> anyhow::Result<Option<Update<'a>>> {
        if self.active {
            let dimensions = self.dimensions;
            let window_dimensions = manager.window_dimensions;
            let mouse_position = manager.mouse_position;

            Ok(Some(Update::new(move |e, event, (em, cm)| {
                if let Ev::Event(Control {
                    event:
                        Event::WindowEvent {
                            event:
                                WindowEvent::MouseInput {
                                    button: MouseButton::Left,
                                    state: s,
                                    ..
                                },
                            ..
                        },
                    flow: _,
                }) = event
                {
                    let p = if let ElementState::Pressed = s {
                        em.entities
                            .keys()
                            .cloned()
                            .find_map(|e| {
                                cm.get::<Camera>(e, em).and_then(|c| c.active.then_some(c))
                            })
                            .and_then(|c| {
                                cm.get::<ScreenPos>(e, em)
                                    .and_then(|t| t.active.then_some(t))
                                    .and_then(|s| {
                                        let transform =
                                            Mat3d::translation(s.position) * Mat3d::scale(s.scale);
                                        let ((max, _), _) =
                                            c.proj() * (transform * ((dimensions / 2.0), 1.0), 1.0);
                                        let ((min, _), _) = c.proj()
                                            * (transform * ((-dimensions / 2.0), 1.0), 1.0);
                                        let mouse_position = Vec2d::new(
                                            mouse_position.0 / window_dimensions.0 as f32 * 2.0
                                                - 1.0,
                                            -(mouse_position.1 / window_dimensions.1 as f32 * 2.0
                                                - 1.0),
                                        );

                                        (mouse_position.x() > min.x()
                                            && mouse_position.x() < max.x()
                                            && mouse_position.y() > min.y()
                                            && mouse_position.y() < max.y())
                                        .then_some(mouse_position)
                                    })
                            })
                    } else {
                        None
                    };

                    if let Some(c) = cm
                        .get_mut::<UiCallback<Vec2d>>(e, em)
                        .and_then(|c| c.active.then_some(c))
                    {
                        c.value = p;
                    }
                };

                Ok(())
            })))
        } else {
            Ok(None)
        }
    }
}
