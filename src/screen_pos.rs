use hex::{
    cid,
    ecs::{component_manager::Component, Id},
    math::Vec2,
};

#[derive(Clone)]
pub struct ScreenPos {
    pub position: Vec2,
    pub scale: Vec2,
    pub active: bool,
}

impl Component for ScreenPos {
    fn id() -> Id {
        cid!()
    }
}
