use super::Ui;
use crate::{ui_manager::State, ScreenPos};
use hex::{
    anyhow,
    components::Camera,
    ecs::{ev::Control, ComponentManager, EntityManager, Ev},
    glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent},
    math::{Mat3d, Vec2d},
};

#[derive(Clone)]
pub struct Button {
    pub dimensions: Vec2d,
    pub active: bool,
}

impl Ui for Button {
    fn ui(
        &self,
        screen_pos: &ScreenPos,
        ev: &Ev,
        state: &State,
        (em, cm): (&EntityManager, &ComponentManager),
    ) -> anyhow::Result<bool> {
        match ev {
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
            }) if self.active => Ok(em
                .entities
                .keys()
                .cloned()
                .find_map(|e| cm.get::<Camera>(e, em).and_then(|c| c.active.then_some(c)))
                .map(|c| {
                    let transform =
                        Mat3d::translation(screen_pos.position) * Mat3d::scale(screen_pos.scale);
                    let ((max, _), _) =
                        c.proj() * (transform * ((self.dimensions / 2.0), 1.0), 1.0);
                    let ((min, _), _) =
                        c.proj() * (transform * ((-self.dimensions / 2.0), 1.0), 1.0);
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
