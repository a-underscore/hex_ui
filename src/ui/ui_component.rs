use super::{Ui, Update};
use crate::UiManager;
use hex::anyhow;

pub struct UiComponent {
    active: bool,
}

impl Ui for UiComponent {
    fn ui(&mut self, _: &mut UiManager) -> anyhow::Result<Update> {
        Ok(Box::new(move |_, _, _| Ok(())))
    }

    fn active(&mut self) -> bool {
        self.active
    }
}
