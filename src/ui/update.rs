use hex::{
    anyhow,
    ecs::{Ev, World},
};

pub type Update = Box<dyn FnMut(usize, &mut Ev, &mut World) -> anyhow::Result<()>>;
