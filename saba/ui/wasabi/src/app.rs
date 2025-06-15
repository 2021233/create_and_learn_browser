use crate::alloc::string::ToString;
use alloc::rc::Rc;
use core::cell::RefCell;
use noli::error::Result as OsResult;
use noli::window::Window;
use noli::windows::StringSize;
use saba_core::browser::Browser;
use saba_core::constants::WINDOW_INIt_X_POS;
use saba_core::constants::WHITE;
use saba_core::constants::WINDOW_HEIGHT;
use saba_core::constants::WINDOW_INIT_Y_POS;
use saba_core::constants::WINDOW_WIDTH;
use saba_core::constants::*;

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Winsow,
}

impl WasabiUI {
    fn setup_toolbar(&mut self) -> OsResult {
    }
}
