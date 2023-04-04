use hex::{
    ecs::{component_manager::Component, Id},
    id,
};

pub struct UiCallback<T> {
    pub value: Option<T>,
    pub active: bool,
}

impl<T> Component for UiCallback<T> {
    fn id() -> Id {
        id!()
    }
}
