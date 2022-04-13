use std::{cell::RefCell, rc::Rc};

use imgui::EditableColor;

thread_local!(static WIDGET_STATE: Rc<RefCell<WidgetState>> = Rc::new(RefCell::new(WidgetState::new())));

pub struct WidgetState {
    pub primary_color: [f32; 4]
}

impl WidgetState {
    fn new() -> Self {
        Self {
            primary_color: [0.0, 0.0, 0.0, 1.0]
        }
    }

    pub fn current() -> Rc<RefCell<WidgetState>> {
        WIDGET_STATE.with(|s| s.clone())
    }
}