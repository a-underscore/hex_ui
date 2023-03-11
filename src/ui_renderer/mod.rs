use crate::ui::Image;
use hex::{
    anyhow,
    assets::Shader,
    components::{Camera, Transform},
    glium::{glutin::event::Event, index::NoIndices, uniform, uniforms::Sampler, Display, Surface},
    hecs::{ev::Control, system_manager::System, Ev, World},
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

impl<'a> System<'a> for UiRenderer {
    fn update(&mut self, event: &mut Ev, world: &mut World<'a>) -> anyhow::Result<()> {
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
                                    .get::<Image>(e, &world.em)
                                    .and_then(|s| s.active.then_some(s))?,
                                world
                                    .cm
                                    .get::<Transform>(e, &world.em)
                                    .and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                    sprites
                };

                let camera_view: [[f32; 4]; 4] = c.view().into();

                for (s, t) in sprites {
                    let color: [f32; 4] = s.color.into();
                    let transform: [[f32; 3]; 3] = t.matrix().into();
                    let uniform = uniform! {
                        z: s.z,
                        transform: transform,
                        camera_view: camera_view,
                        color: color,
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
