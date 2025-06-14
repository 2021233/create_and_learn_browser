use alloc::rc::Rc;
use core::cell::RefCell:
use noli::window::Window;
use saba_core::browser::Browser;
use crate::alloc::string::ToString;
use saba_core::constants::WHITE;
use saba_core::constants::WINDOW_HEIGHT;
use saba_core::constants::WINDOW_INIt_X_POS;
use saba_core::constants::WINDOW_INIT_Y_POS;
use saba_core::constants::WINDOW_WIDTH;

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Winsow,
}
