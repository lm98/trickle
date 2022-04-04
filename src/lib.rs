extern "C" {
    fn js_clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32);
}

pub fn clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        js_clear_screen_to_color(red, green, blue, alpha)
    }
}

#[no_mangle]
pub extern "C" fn key_pressed() {
    clear_screen_to_color(0.0, 1.0, 0.0, 1.0);
}

thread_local! {
    pub static EVENT_HANDLER: std::cell::RefCell<Box<dyn FnMut()>> 
        = std::cell::RefCell::new(Box::new(||{}));
}

pub fn set_event_handler(function: impl FnMut() + 'static) {
    EVENT_HANDLER.with(|event_handler| {
        *event_handler.borrow_mut() = Box::new(function);
    });
}