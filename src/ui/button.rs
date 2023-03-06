use super::Ui;
use crate::{Callback, UiManager};
use hex::{
    anyhow,
    cgmath::Vector2,
    components::{Camera, Transform},
    glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent},
    hecs::{ev::Control, Ev},
};

pub struct Button {
    pub max: Vector2<f32>,
    pub min: Vector2<f32>,
    pub events: Vec<usize>,
}

impl Ui for Button {
    fn update(
        &mut self,
        e: usize,
        event: &mut Ev,
        manager: &mut UiManager,
    ) -> anyhow::Result<Option<Callback>> {
        let _mouse_pos = manager.mouse_pos;
        let _window_dims = manager.window_dims;

        Ok(match event {
            Ev::Event(Control {
                event:
                    Event::WindowEvent {
                        event:
                            WindowEvent::MouseInput {
                                button: MouseButton::Left,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                flow: _,
            }) => {
                self.events.push(e);

                Some(Box::new(|_e, _ev, world| {
                    if let Some((_c, _ct)) = world.em.entities.keys().cloned().find_map(|e| {
                        Some((
                            world
                                .cm
                                .get::<Camera>(e, &world.em)
                                .and_then(|c| c.active.then_some(c))?,
                            world
                                .cm
                                .get::<Transform>(e, &world.em)
                                .and_then(|t| t.active.then_some(t))?,
                        ))
                    }) {
                        // TODO: find on-screen position and check if it is within the dimensions of
                        // the bounding box
                    }

                    Ok(())
                }))
            }
            _ => None,
        })
    }
}
