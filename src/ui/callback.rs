use hex::{
    ecs::{component_manager::Component, Id},
    id,
};

pub struct Callback {
    checked: bool,
    value: bool,
    pub active: bool,
}

impl Callback {
    pub fn set(&mut self, value: bool) {
        if !self.checked {
            self.checked = false;

            self.value = value;
        }
    }

    pub fn check(&mut self) -> bool {
        self.checked = true;

        self.value
    }
}

impl Component for Callback {
    fn id() -> Id {
        id!()
    }
}
