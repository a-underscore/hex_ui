use hex::{
    cid,
    ecs::{component_manager::Component, Id},
};

pub struct UiCallback<T> {
    pub value: Option<T>,
    pub active: bool,
}

impl<T> Component for UiCallback<T> {
    fn id() -> Id {
        cid!()
    }
}
