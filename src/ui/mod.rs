pub mod button;
pub mod callback;

pub use callback::Callback;

use crate::UiManager;
use hex::{
    cid,
    hecs::{component_manager::Component, Ev},
};

pub trait Ui {
    fn update(&mut self, _: &mut Ev, _: &mut UiManager) -> Callback {
        Ok(None)
    }
}

impl Component for Box<dyn Ui> {
    fn id() -> usize {
        cid!()
    }
}
