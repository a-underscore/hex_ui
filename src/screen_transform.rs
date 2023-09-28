use hex::{
    ecs::component_manager::Component,
    math::{Mat3d, Vec2d},
};

#[derive(Clone)]
pub struct ScreenTransform {
    pub position: Vec2d,
    pub rotation: f32,
    pub scale: Vec2d,
    pub active: bool,
}

impl ScreenTransform {
    pub fn new(position: Vec2d, rotation: f32, scale: Vec2d, active: bool) -> Self {
        Self {
            position,
            rotation,
            scale,
            active,
        }
    }

    pub fn matrix(&self) -> Mat3d {
        Mat3d::translation(self.position)
            * Mat3d::rotation(self.rotation)
            * Mat3d::scale(self.scale)
    }
}

impl Component for ScreenTransform {}
