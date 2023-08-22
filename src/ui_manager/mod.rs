pub mod state;

pub use state::State;

use crate::{ui::Callback, Ui};
use hex::{
    anyhow,
    ecs::{ev::Control, system_manager::System, ComponentManager, Context, EntityManager, Ev},
    glium::glutin::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, WindowEvent},
    },
};

#[derive(Default)]
pub struct UiManager {
    pub state: State,
}

impl System<'_> for UiManager {
    fn update(
        &mut self,
        ev: &mut Ev,
        _: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        match ev {
            Ev::Event(Control {
                event:
                    Event::WindowEvent {
                        event:
                            WindowEvent::Resized(PhysicalSize {
                                width: x,
                                height: y,
                            }),
                        window_id: _,
                    },
                flow: _,
            }) => {
                self.state.window_dimensions = (*x, *y);
            }
            Ev::Event(Control {
                event:
                    Event::WindowEvent {
                        event:
                            WindowEvent::CursorMoved {
                                position: PhysicalPosition { x, y },
                                ..
                            },
                        window_id: _,
                    },
                flow: _,
            }) => {
                self.state.mouse_position = (*x as f32, *y as f32);
            }
            _ => {}
        }

        for e in em.entities.keys().cloned() {
            if let Some(u) = cm
                .get::<Box<dyn Ui>>(e, em)
                .map(|u| u.ui(e, ev, &self.state, (em, cm)))
            {
                let u = u?;

                if let Some(c) = cm.get_mut::<Callback>(e, em) {
                    c.set(u);
                }
            }
        }

        Ok(())
    }
}
