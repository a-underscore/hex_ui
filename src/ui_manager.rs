use super::Ui;
use hex::{
    anyhow,
    hecs::{ev::Ev, system_manager::System, world::World},
};

pub struct UiManager;

impl System<'_> for UiManager {
    fn update(&mut self, ev: &mut Ev, world: &mut World) -> anyhow::Result<()> {
        for e in world.em.entities.keys().cloned().collect::<Vec<_>>() {
            if let Some(ui) = world.cm.get::<Ui>(e, &world.em).map(|Ui(f)| f.clone()) {
                ui(e, ev, world, self);
            }
        }

        Ok(())
    }
}
