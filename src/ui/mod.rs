pub mod button;
pub mod update;

pub use button::Button;
pub use update::Update;

use crate::UiManager;
use hex::{
    anyhow,
    ecs::{component_manager::Component, Ev, Id},
    id,
};

pub trait Ui {
    fn ui<'a>(
        &mut self,
        ev: &mut Ev,
        manager: &mut UiManager,
    ) -> anyhow::Result<Option<Update<'a>>>;
}

impl Component for Box<dyn Ui> {
    fn id() -> Id {
        id!()
    }
}
