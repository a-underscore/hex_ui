pub mod button;
pub mod image;
pub mod text;
pub mod ui_callback;
pub mod update;

pub use button::Button;
pub use image::Image;
pub use ui_callback::UiCallback;
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
