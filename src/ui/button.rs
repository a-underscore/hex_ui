use super::Ui;
use crate::{ui_manager::State, ScreenTransform};
use hex::{
    anyhow,
    components::Camera,
    ecs::{ev::Control, ComponentManager, EntityManager, Ev, Id},
    glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent},
    math::Vec2d,
};

#[derive(Clone)]
pub struct Button {
    pub dimensions: Vec2d,
    pub active: bool,
}

impl Ui for Button {
    fn ui(
        &self,
        e: Id,
        ev: &Ev,
        state: &State,
        (em, cm): (&EntityManager, &ComponentManager),
    ) -> anyhow::Result<bool> {
        match (ev, cm.get::<ScreenTransform>(e, em)) {
            (
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
                }),
                Some(transform),
            ) if self.active => Ok(em
                .entities
                .keys()
                .cloned()
                .find_map(|e| cm.get::<Camera>(e, em).and_then(|c| c.active.then_some(c)))
                .map(|c| {
                    let matrix = transform.matrix();
                    let ((max, _), _) = c.proj() * (matrix * ((self.dimensions / 2.0), 1.0), 1.0);
                    let ((min, _), _) = c.proj() * (matrix * ((-self.dimensions / 2.0), 1.0), 1.0);
                    let mouse_position = Vec2d::new(
                        state.mouse_position.0 / state.window_dimensions.0 as f32 * 2.0 - 1.0,
                        -(state.mouse_position.1 / state.window_dimensions.1 as f32 * 2.0 - 1.0),
                    );

                    mouse_position.x() > min.x()
                        && mouse_position.x() < max.x()
                        && mouse_position.y() > min.y()
                        && mouse_position.y() < max.y()
                })
                .unwrap_or_default()),
            _ => Ok(false),
        }
    }
}
