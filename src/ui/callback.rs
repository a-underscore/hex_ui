use hex::{cid, ecs::component_manager::Component};

pub struct Callback<T> {
    pub value: Option<T>,
    pub active: bool,
}

impl<T> Component for Callback<T> {
    fn id() -> usize {
        cid!()
    }
}
