use super::{Image, Ui, Update};
use crate::UiManager;
use hex::{
    ecs::{ev::Control, Ev},
    glium::glutin::event::Event,
};

pub struct Text {
    pub content: String,
    pub active: bool,
}

impl Ui for Text {
    fn ui<'a>(&mut self, _: &mut UiManager) -> Option<Update<'a>> {
        if self.active {
            Some(Update::new(|e, event, world| {
                if let Ev::Event(Control {
                    event: Event::MainEventsCleared,
                    flow: _,
                }) = event
                {
                    if let Some(_) = world.cm.get::<Image>(e, &world.em) {
                        todo!()
                    }
                }

                Ok(())
            }))
        } else {
            None
        }
    }
}
