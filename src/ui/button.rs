use super::{Callback, Ui, Update};
use crate::UiManager;
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
}

impl Button {}

impl Ui for Button {
    fn ui(&mut self, _manager: &mut UiManager) -> anyhow::Result<Update> {
        Ok(Box::new(|e, event, world| {
            if let Ev::Event(Control {
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
            }) = event
            {
                let p = world
                    .em
                    .entities
                    .keys()
                    .cloned()
                    .find_map(|e| {
                        Some((
                            world
                                .cm
                                .get::<Camera>(e, &world.em)
                                .and_then(|c| c.active.then_some(c.clone()))?,
                            world
                                .cm
                                .get::<Transform>(e, &world.em)
                                .and_then(|t| t.active.then_some(t.clone()))?,
                        ))
                    })
                    .and_then(|_| /* TODO */ todo!());

                if let Some(Callback(ref mut mp)) =
                    world.cm.get_mut::<Callback<Vector2<f32>>>(e, &world.em)
                {
                    *mp = p;
                }
            }
            Ok(())
        }))
    }
}
