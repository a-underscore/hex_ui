use hex::{
    anyhow,
    hecs::{Ev, World},
};

pub type Callback = anyhow::Result<Option<Box<dyn FnMut(usize, &mut Ev, &mut World)>>>;
