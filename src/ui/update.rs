use hex::{
    anyhow,
    ecs::{Ev, World},
};

pub type UpdateFn<'a> = Box<dyn FnMut(usize, &mut Ev, &mut World) -> anyhow::Result<()> + 'a>;

pub struct Update<'a>(pub UpdateFn<'a>);

impl<'a> Update<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut(usize, &mut Ev, &mut World) -> anyhow::Result<()> + 'a,
    {
        Self(Box::new(f))
    }
}
