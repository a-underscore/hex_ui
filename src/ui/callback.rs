use hex::{cid, hecs::component_manager::Component};

pub struct Callback<T>(pub Vec<(usize, T)>);

impl<T> Component for Callback<T> {
    fn id() -> usize {
        cid!()
    }
}
