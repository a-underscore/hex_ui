use crate::Ui;
use hex::{
    anyhow,
    components::{Camera, Transform},
    glium::glutin::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, WindowEvent},
    },
    hecs::{ev::Control, system_manager::System, world::World, Ev},
};

#[derive(Default)]
pub struct UiManager {
    pub window_dimensions: (u32, u32),
    pub mouse_position: (f32, f32),
}

impl System<'_> for UiManager {
    fn update(&mut self, ev: &mut Ev, world: &mut World) -> anyhow::Result<()> {
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
                self.window_dimensions = (*x, *y);
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
                self.mouse_position = (*x as f32, *y as f32);
            }
            _ => {}
        }

        if let Some((_, _)) = world.em.entities.keys().cloned().find_map(|e| {
            Some((
                world
                    .cm
                    .get::<Camera>(e, &world.em)
                    .and_then(|c| c.active.then_some(c))?
                    .clone(),
                world
                    .cm
                    .get::<Transform>(e, &world.em)
                    .and_then(|t| t.active.then_some(t))?
                    .clone(),
            ))
        }) {
            for e in world.em.entities.keys().cloned().collect::<Vec<_>>() {
                if let Some(u) = world
                    .cm
                    .get_mut::<Box<dyn Ui>>(e, &world.em)
                    .and_then(|u| u.active().then_some(u))
                    .map(|u| u.ui(self).map(|c| (e, c)))
                {
                    let (e, mut u) = u?;

                    u(e, ev, world)?;
                }
            }
        }

        Ok(())
    }
}
