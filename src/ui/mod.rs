use crate::UiManager;
use hex::{
    cid,
    hecs::{component_manager::Component, Ev, World},
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Ui(pub Rc<dyn Fn(usize, &mut Ev, &mut World, &mut UiManager)>);

impl Component for Ui {
    fn id() -> usize {
        cid!()
    }
}
