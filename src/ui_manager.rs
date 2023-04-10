use crate::Ui;
use hex::{
    anyhow,
    ecs::{ev::Control, system_manager::System, ComponentManager, EntityManager, Ev, Scene},
    glium::glutin::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, WindowEvent},
    },
};

#[derive(Default)]
pub struct UiManager {
    pub window_dimensions: (u32, u32),
    pub mouse_position: (f32, f32),
}

impl System<'_> for UiManager {
    fn update(
        &mut self,
        event: &mut Ev,
        _: &mut Scene,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        match event {
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

        for (e, u) in em
            .entities
            .keys()
            .cloned()
            .filter_map(|e| {
                Some(
                    cm.get_mut::<Box<dyn Ui>>(e, &em)
                        .map(|u| u.ui(event, self))?
                        .map(|u| (e, u)),
                )
            })
            .collect::<anyhow::Result<Vec<_>>>()?
        {
            {
                if let Some(mut u) = u {
                    u.0(e, event, (em, cm))?;
                }
            }
        }

        Ok(())
    }
}
