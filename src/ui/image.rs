use hex::{cid, components::Sprite, ecs::component_manager::Component};

#[derive(Clone)]
pub struct Image<'a>(pub Sprite<'a>);

impl<'a> Component for Image<'a> {
    fn id() -> usize {
        cid!()
    }
}
