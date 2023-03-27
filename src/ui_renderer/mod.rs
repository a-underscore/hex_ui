use crate::ScreenPos;
use hex::{
    anyhow,
    assets::Shader,
    components::{Camera, Sprite},
    ecs::{ev::Control, system_manager::System, Ev, Scene, World},
    glium::{glutin::event::Event, index::NoIndices, uniform, uniforms::Sampler, Display, Surface},
    math::Mat3,
};

pub static UI_VERTEX_SRC: &str = include_str!("ui_vertex.glsl");
pub static UI_FRAGMENT_SRC: &str = include_str!("ui_fragment.glsl");

pub struct UiRenderer {
    pub shader: Shader,
}

impl UiRenderer {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            shader: Shader::new(display, UI_VERTEX_SRC, UI_FRAGMENT_SRC, None)?,
        })
    }
}

impl System<'_> for UiRenderer {
    fn update(&mut self, event: &mut Ev, _: &mut Scene, world: &mut World) -> anyhow::Result<()> {
        if let Ev::Draw((
            Control {
                event: Event::MainEventsCleared,
                flow: _,
            },
            target,
        )) = event
        {
            if let Some(c) = world.em.entities.keys().cloned().find_map(|e| {
                world
                    .cm
                    .get::<Camera>(e, &world.em)
                    .and_then(|c| c.active.then_some(c))
            }) {
                target.clear_depth(1.0);

                let sprites = {
                    let mut sprites: Vec<_> = world
                        .em
                        .entities
                        .keys()
                        .cloned()
                        .filter_map(|e| {
                            Some((
                                world
                                    .cm
                                    .get::<Sprite>(e, &world.em)
                                    .and_then(|s| s.active.then_some(s))?,
                                world
                                    .cm
                                    .get::<ScreenPos>(e, &world.em)
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
                        transform: (Mat3::translation(t.position) * Mat3::scale(t.scale)).0,
                        camera_view: c.view().0,
                        color: s.color,
                        tex: Sampler(&*s.texture.buffer, s.texture.sampler_behaviour),
                    };

                    target.draw(
                        &*s.shape.vertices,
                        NoIndices(s.shape.format),
                        &self.shader.program,
                        &uniform,
                        &s.draw_parameters,
                    )?;
                }
            }
        }

        Ok(())
    }
}
