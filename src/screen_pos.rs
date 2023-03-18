use hex::{cid, ecs::component_manager::Component, math::Vec2};

pub struct ScreenPos {
    pub position: Vec2,
    pub scale: Vec2,
    pub active: bool,
}

impl Component for ScreenPos {
    fn id() -> usize {
        cid!()
    }
}
