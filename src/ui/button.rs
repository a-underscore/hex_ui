use super::{Ui, UiCallback, Update};
use crate::UiManager;
use hex::{
    anyhow,
    components::{Camera, Transform},
    ecs::{ev::Control, Ev},
    glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent},
    math::Vec2,
};

#[derive(Clone)]
pub struct Button {
    pub dimensions: Vec2,
    pub active: bool,
}

impl Ui for Button {
    fn ui(&mut self, manager: &mut UiManager) -> anyhow::Result<Update> {
        let dimensions = self.dimensions;
        let window_dimensions = manager.window_dimensions;
        let mouse_position = manager.mouse_position;

        Ok(Box::new(move |e, event, world| {
            let p = if let Ev::Event(Control {
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
                world
                    .em
                    .entities
                    .keys()
                    .cloned()
                    .find_map(|e| {
                        world
                            .cm
                            .get::<Camera>(e, &world.em)
                            .and_then(|c| c.active.then_some(c))
                    })
                    .and_then(|c| {
                        let p = world.cm.get::<Transform>(e, &world.em).and_then(|s| {
                            let ((max, _), _) =
                                c.view() * (s.matrix() * ((dimensions / 2.0), 1.0), 1.0);
                            let ((min, _), _) =
                                c.view() * (s.matrix() * ((-dimensions / 2.0), 1.0), 1.0);
                            let mouse_position = Vec2::new(
                                mouse_position.0 / window_dimensions.0 as f32 * 2.0 - 1.0,
                                -(mouse_position.1 / window_dimensions.1 as f32 * 2.0 - 1.0),
                            );

                            (mouse_position.x() > min.x()
                                && mouse_position.x() < max.x()
                                && mouse_position.y() > min.y()
                                && mouse_position.y() < max.y())
                            .then_some(mouse_position)
                        });

                        p
                    })
            } else {
                None
            };

            if let Some(c) = world
                .cm
                .get_mut::<UiCallback<Vec2>>(e, &world.em)
                .and_then(|c| c.active.then_some(c))
            {
                c.value = p;
            }
            Ok(())
        }))
    }

    fn active(&mut self) -> bool {
        self.active
    }
}
