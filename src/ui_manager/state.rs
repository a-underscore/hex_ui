#[derive(Clone, Default)]
pub struct State {
    pub window_dimensions: (u32, u32),
    pub mouse_position: (f32, f32),
}

impl State {
    pub fn new(window_dimensions: (u32, u32), mouse_position: (f32, f32)) -> Self {
        Self {
            window_dimensions,
            mouse_position,
        }
    }
}
