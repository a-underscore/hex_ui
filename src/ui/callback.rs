use hex::{cid, hecs::component_manager::Component};

#[derive(Default)]
pub struct Callback<T> {
    pub value: Option<T>,
}

impl<T> Component for Callback<T> {
    fn id() -> usize {
        cid!()
    }
}
