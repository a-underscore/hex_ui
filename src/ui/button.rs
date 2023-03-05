use super::{Callback, Ui};
use crate::UiManager;
use hex::{cgmath::Vector2, hecs::Ev};

pub struct Button {
    pub max: Vector2<f32>,
    pub min: Vector2<f32>,
}

impl Ui for Button {
    fn update(&mut self, _: &mut Ev, _: &mut UiManager) -> Callback {
        Ok(None)
    }
}
