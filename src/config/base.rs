use dotenv::dotenv;
use paste::paste;
use std::borrow::BorrowMut;
use std::sync::{Mutex, Once};

macro_rules! config_key_wrapper {
    ($name:ident, $variable_name:ident) => {
        paste! {
            static mut [<STD_ONCE_ $name>]: Option<Mutex<String>> = None;
            static [<$name _INIT>]: Once = Once::new();

            pub fn $variable_name<'a>() -> &'a Mutex<String> {
                [<$name _INIT>].call_once(|| unsafe {
                    *[<STD_ONCE_ $name>].borrow_mut() = Some(Mutex::new(
                        std::env::var(stringify!($name)).expect(format!("{:?} must be set.", stringify!($name)).as_str()),
                    ));
                });
                unsafe { [<STD_ONCE_ $name>].as_ref().unwrap() }
            }
        }
    };
}

config_key_wrapper!(DNS_KEY, dns_key);

pub fn init() {
    dotenv().ok();
    *dns_key().lock().unwrap() = std::env::var("DNS_KEY").expect("DNS_KEY must be set.");
}
