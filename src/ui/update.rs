use hex::{
    anyhow,
    ecs::{Ev, Id, World},
};

pub type UpdateFn<'a> = Box<dyn FnMut(Id, &mut Ev, &mut World) -> anyhow::Result<()> + 'a>;

pub struct Update<'a>(pub UpdateFn<'a>);

impl<'a> Update<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut(Id, &mut Ev, &mut World) -> anyhow::Result<()> + 'a,
    {
        Self(Box::new(f))
    }
}
