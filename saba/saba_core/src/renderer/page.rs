use crate::browser::Browser;
use crate::renderer::dom::node::Window;
use alloc::rc::Rc;
use alloc::rc::Weak;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Page {
    browser: Weak<RefCell<Browser>>,
    frame: Option<Rc<RefCell<Window>>>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            browser: Weak::new(),
            frame: None,
        }
    }

    pub fn set_browser(&mut self, browser: Weak<RefCell<Browser>>) {
        self.browser = browser;
    }
}
