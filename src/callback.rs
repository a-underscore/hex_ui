use hex::{
    anyhow,
    hecs::{Ev, World},
};

pub type Callback = Box<dyn FnMut(usize, &mut Ev, &mut World) -> anyhow::Result<()>>;
