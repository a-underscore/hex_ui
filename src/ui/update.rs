use hex::{
    anyhow,
    ecs::{ComponentManager, EntityManager, Ev, Id},
};

pub type UpdateFn<'a> =
    Box<dyn FnMut(Id, &mut Ev, (&EntityManager, &mut ComponentManager)) -> anyhow::Result<()> + 'a>;

pub struct Update<'a>(pub UpdateFn<'a>);

impl<'a> Update<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut(Id, &mut Ev, (&EntityManager, &mut ComponentManager)) -> anyhow::Result<()> + 'a,
    {
        Self(Box::new(f))
    }
}
