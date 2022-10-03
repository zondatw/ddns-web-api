use dotenv::dotenv;
use std::borrow::BorrowMut;
use std::sync::{Mutex, Once};

static mut STD_ONCE_DNS_KEY: Option<Mutex<String>> = None;
static DNS_KEY_INIT: Once = Once::new();

pub fn dns_key<'a>() -> &'a Mutex<String> {
    DNS_KEY_INIT.call_once(|| unsafe {
        *STD_ONCE_DNS_KEY.borrow_mut() = Some(Mutex::new(
            std::env::var("DNS_KEY").expect("DNS_KEY must be set."),
        ));
    });
    unsafe { STD_ONCE_DNS_KEY.as_ref().unwrap() }
}

pub fn init() {
    dotenv().ok();
    *dns_key().lock().unwrap() = std::env::var("DNS_KEY").expect("DNS_KEY must be set.");
}
