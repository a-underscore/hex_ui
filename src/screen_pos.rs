use hex::{cgmath::Vector2, cid, hecs::component_manager::Component};

#[derive(Clone)]
pub struct ScreenPos {
    pub position: Vector2<f32>,
    pub active: bool,
}

impl Component for ScreenPos {
    fn id() -> usize {
        cid!()
    }
}
