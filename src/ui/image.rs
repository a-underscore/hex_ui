use hex::{
    assets::{Shape, Texture},
    cgmath::Vector4,
    cid,
    glium::{
        draw_parameters::{Blend, DepthTest},
        Depth, DrawParameters,
    },
    hecs::component_manager::Component,
};

#[derive(Clone)]
pub struct Image<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub shape: Shape,
    pub texture: Texture,
    pub color: Vector4<f32>,
    pub z: f32,
    pub active: bool,
}

impl<'a> Image<'a> {
    pub fn new(shape: Shape, texture: Texture, color: Vector4<f32>, z: f32, active: bool) -> Self {
        Self {
            draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                ..Default::default()
            },
            shape,
            texture,
            color,
            z,
            active,
        }
    }
}

impl<'a> Component for Image<'a> {
    fn id() -> usize {
        cid!()
    }
}
