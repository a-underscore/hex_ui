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
use hex::{cid, ecs::component_manager::Component};

pub trait Ui {
    fn ui<'a>(&mut self, manager: &mut UiManager) -> Option<Update<'a>>;
}

impl Component for Box<dyn Ui> {
    fn id() -> usize {
        cid!()
    }
}
