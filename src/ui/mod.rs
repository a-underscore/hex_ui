pub mod button;
pub mod update;

pub use button::Button;
pub use update::Update;

use crate::UiManager;
use hex::{
    anyhow, cid,
    ecs::{component_manager::Component, Ev},
};

pub trait Ui {
    fn ui<'a>(
        &mut self,
        event: &mut Ev,
        manager: &mut UiManager,
    ) -> anyhow::Result<Option<Update<'a>>>;
}

impl Component for Box<dyn Ui> {
    fn id() -> usize {
        cid!()
    }
}
