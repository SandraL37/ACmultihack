use std::{cell::RefCell, sync::LazyLock};

use crate::gui::gui::Gui;

pub mod gui;
pub mod style;

thread_local! {
    pub static GUI: LazyLock<RefCell<Gui>> = LazyLock::new(|| {
        RefCell::new(Gui::new())
    });
}