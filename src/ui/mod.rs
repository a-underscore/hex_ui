pub mod button;
pub mod callback;
pub mod image;
pub mod update;

pub use button::Button;
pub use callback::Callback;
pub use image::Image;
pub use update::Update;

use crate::UiManager;
use hex::{anyhow, cid, ecs::component_manager::Component};

pub trait Ui {
    fn ui(&mut self, manager: &mut UiManager) -> anyhow::Result<Update>;

    fn active(&mut self) -> bool;
}

impl Component for Box<dyn Ui> {
    fn id() -> usize {
        cid!()
    }
}
