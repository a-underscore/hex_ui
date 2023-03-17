use hex::{
    anyhow,
    ecs::{Ev, World},
};

pub struct Update<'a>(pub Box<dyn FnMut(usize, &mut Ev, &mut World) -> anyhow::Result<()> + 'a>);

impl<'a> Update<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut(usize, &mut Ev, &mut World) -> anyhow::Result<()> + 'a,
    {
        Self(Box::new(f))
    }
}
