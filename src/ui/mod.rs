pub mod button;
pub mod callback;
pub mod update;

pub use button::Button;
pub use callback::Callback;
pub use update::Update;

use crate::ui_manager::State;
use hex::{
    anyhow,
    ecs::{component_manager::Component, ComponentManager, EntityManager, Ev, Id},
    id,
};

pub trait Ui {
    fn ui(
        &self,
        id: Id,
        ev: &Ev,
        state: &State,
        world: (&EntityManager, &ComponentManager),
    ) -> anyhow::Result<bool>;
}

impl Component for Box<dyn Ui> {
    fn id() -> Id {
        id!()
    }
}
