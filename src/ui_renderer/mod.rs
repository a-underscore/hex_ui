use crate::ScreenTransform;
use hex::{
    anyhow,
    assets::Shader,
    components::Sprite,
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
    glium::{
        draw_parameters::{Blend, DepthTest},
        index::NoIndices,
        uniform,
        uniforms::Sampler,
        Depth, Display, DrawParameters, Surface,
    },
    math::Ortho,
};

pub struct UiRenderer {
    pub ortho: Ortho,
    pub draw_parameters: DrawParameters<'static>,
    pub shader: Shader,
}

impl UiRenderer {
    pub fn new(display: &Display, ortho: Ortho) -> anyhow::Result<Self> {
        Ok(Self {
            ortho,
            draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                ..Default::default()
            },
            shader: Shader::new(
                display,
                include_str!("ui_vertex.glsl"),
                include_str!("ui_fragment.glsl"),
                None,
            )?,
        })
    }
}

impl System for UiRenderer {
    fn update(
        &mut self,
        ev: &mut Ev,
        _: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, target)) = ev {
            target.clear_depth(1.0);

            let sprites = {
                let mut sprites: Vec<_> = em
                    .entities
                    .keys()
                    .cloned()
                    .filter_map(|e| {
                        Some((
                            cm.get::<Sprite>(e, em)
                                .and_then(|s| s.active.then_some(s))?,
                            cm.get::<ScreenTransform>(e, em)
                                .and_then(|t| t.active.then_some(t))?,
                        ))
                    })
                    .collect();

                sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                sprites
            };

            for (s, t) in sprites {
                let uniform = uniform! {
                    z: s.z,
                    transform: t.matrix().0,
                    camera_proj: self.ortho.0,
                    color: s.color,
                    tex: Sampler(&*s.texture.buffer, s.texture.sampler_behaviour),
                };

                target.draw(
                    &*s.shape.vertices,
                    NoIndices(s.shape.format),
                    &self.shader.program,
                    &uniform,
                    &self.draw_parameters,
                )?;
            }
        }

        Ok(())
    }
}
