use super::{Callback, Ui, Update};
use crate::UiManager;
use hex::{
    anyhow,
    cgmath::Vector2,
    components::{Camera, Transform},
    glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent},
    hecs::{ev::Control, Ev},
};

#[derive(Clone)]
pub struct Button {
    pub dimensions: Vector2<f32>,
    pub active: bool,
}

impl Ui for Button {
    fn ui(&mut self, manager: &mut UiManager) -> anyhow::Result<Update> {
        let dimensions = self.dimensions;
        let window_dimensions = manager.window_dimensions;
        let mouse_position = manager.mouse_position;

        Ok(Box::new(move |e, event, world| {
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
                if let Some(c) = world.em.entities.keys().cloned().find_map(|e| {
                    world
                        .cm
                        .get::<Camera>(e, &world.em)
                        .and_then(|c| c.active.then_some(c))
                }) {
                    let p = world.cm.get::<Transform>(e, &world.em).and_then(|s| {
                        let max = (c.view()
                            * (s.matrix() * (dimensions / 2.0).extend(1.0)).extend(1.0))
                        .xy();
                        let min = (c.view()
                            * (s.matrix() * (-dimensions / 2.0).extend(1.0)).extend(1.0))
                        .xy();
                        let mouse_position = Vector2::new(
                            mouse_position.0 / window_dimensions.0 as f32 * 2.0 - 1.0,
                            mouse_position.1 / window_dimensions.1 as f32 * 2.0 - 1.0,
                        );

                        (mouse_position.x > min.x
                            && mouse_position.x < max.x
                            && mouse_position.y > min.y
                            && mouse_position.y < max.y)
                            .then_some(mouse_position)
                    });

                    if let Some(c) = world.cm.get_mut::<Callback<Vector2<f32>>>(e, &world.em) {
                        c.value = p;
                    }
                }
            }

            Ok(())
        }))
    }

    fn active(&mut self) -> bool {
        self.active
    }
}
