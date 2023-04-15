use hex::{
    ecs::{component_manager::Component, Id},
    id,
    math::Vec2d,
};

#[derive(Clone)]
pub struct ScreenPos {
    pub position: Vec2d,
    pub scale: Vec2d,
    pub active: bool,
}

impl Component for ScreenPos {
    fn id() -> Id {
        id!()
    }
}
