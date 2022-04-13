use std::{cell::RefCell, rc::Rc};

use crate::color::Color;

thread_local!(static WIDGET_STATE: Rc<RefCell<WidgetState>> = Rc::new(RefCell::new(WidgetState::new())));

pub struct WidgetState {
    pub primary_color: Color,
    pub thickness: u32
}

impl WidgetState {
    fn new() -> Self {
        Self {
            primary_color: Color::Float4 { r: 0.0, g: 0.0, b: 0.0, a: 255.0 },
            thickness: 2
        }
    }

    pub fn current() -> Rc<RefCell<WidgetState>> {
        WIDGET_STATE.with(|s| s.clone())
    }
}