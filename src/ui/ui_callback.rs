use hex::{cid, ecs::component_manager::Component};

pub struct UiCallback<T> {
    pub value: Option<T>,
    pub active: bool,
}

impl<T> Component for UiCallback<T> {
    fn id() -> usize {
        cid!()
    }
}
