use hex::{cid, hecs::component_manager::Component};

pub struct Callback<T>(pub Option<T>);

impl<T> Component for Callback<T> {
    fn id() -> usize {
        cid!()
    }
}
