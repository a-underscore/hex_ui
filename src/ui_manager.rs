use crate::{ScreenPos, Ui};
use hex::{
    anyhow,
    cgmath::Vector2,
    components::{Camera, Transform},
    hecs::{ev::Ev, system_manager::System, world::World},
};

pub struct UiManager;

impl UiManager {
    fn screen_to_world_pos(
        camera: &Camera,
        transform: &Transform,
        screen_pos: &ScreenPos,
    ) -> Vector2<f32> {
        let position = screen_pos.pos;
        let dims = camera.dimensions();
        let on_screen_pos = Vector2::new(position.x / dims.x, position.y / dims.y);

        transform.position() + on_screen_pos
    }
}

impl System<'_> for UiManager {
    fn update(&mut self, ev: &mut Ev, world: &mut World) -> anyhow::Result<()> {
        if let Some((c, ct)) = world.em.entities.keys().cloned().find_map(|e| {
            Some((
                world
                    .cm
                    .get::<Camera>(e, &world.em)
                    .and_then(|c| c.active.then_some(c))?
                    .clone(),
                world
                    .cm
                    .get::<Transform>(e, &world.em)
                    .and_then(|t| t.active.then_some(t))?
                    .clone(),
            ))
        }) {
            for (e, c) in world
                .em
                .entities
                .keys()
                .cloned()
                .into_iter()
                .filter_map(|e| {
                    world
                        .cm
                        .get::<ScreenPos>(e, &world.em)
                        .cloned()
                        .and_then(|s| {
                            let transform = world.cm.get_mut::<Transform>(e, &world.em)?;

                            transform.set_position(Self::screen_to_world_pos(&c, &ct, &s));

                            world
                                .cm
                                .get_mut::<Box<dyn Ui>>(e, &world.em)
                                .map(|u| u.update(ev, self).map(|c| (e, c)))
                        })
                })
                .collect::<anyhow::Result<Vec<_>>>()?
            {
                if let Some(mut callback) = c {
                    callback(e, ev, world);
                }
            }
        }

        Ok(())
    }
}
