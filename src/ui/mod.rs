pub mod button;

pub use crate::callback::Callback;

use crate::UiManager;
use hex::{
    anyhow, cid,
    hecs::{component_manager::Component, Ev},
};

pub trait Ui {
    fn update(
        &mut self,
        _: usize,
        _: &mut Ev,
        _: &mut UiManager,
    ) -> anyhow::Result<Option<Callback>> {
        Ok(None)
    }
}

impl Component for Box<dyn Ui> {
    fn id() -> usize {
        cid!()
    }
}
