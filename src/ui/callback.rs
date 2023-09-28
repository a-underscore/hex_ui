use hex::ecs::component_manager::Component;

#[derive(Default)]
pub struct Callback {
    value: bool,
    pub active: bool,
}

impl Callback {
    pub fn set(&mut self) {
        self.value = true;
    }

    pub fn check(&mut self) -> bool {
        let value = self.value;

        self.value = false;

        value
    }
}

impl Component for Callback {}
