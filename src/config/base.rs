use dotenv::dotenv;
use paste::paste;
use std::borrow::BorrowMut;
use std::sync::{Mutex, Once};

macro_rules! config_key_wrapper {
    ($name:ident, $variable_name:ident, $typ:ty) => {
        paste! {
            static mut [<STD_ONCE_ $name>]: Option<Mutex<$typ>> = None;
            static [<$name _INIT>]: Once = Once::new();

            pub fn $variable_name<'a>() -> &'a Mutex<$typ> {
                [<$name _INIT>].call_once(|| unsafe {
                    *[<STD_ONCE_ $name>].borrow_mut() = Some(Mutex::new(
                        std::env::var(stringify!($name)).expect(format!("{:?} must be set.", stringify!($name)).as_str())
                        .parse::<$typ>()
                        .expect(format!("{:?} must be {:?}.", stringify!($name), stringify!($typ)).as_str()),
                    ));
                });
                unsafe { [<STD_ONCE_ $name>].as_ref().unwrap() }
            }
        }
    };
}

config_key_wrapper!(SERVER_DOMAIN, server_domain, String);
config_key_wrapper!(SERVER_PORT, server_port, u16);
config_key_wrapper!(DNS_KEY, dns_key, String);
config_key_wrapper!(DNS_SERVER, dns_server, String);
config_key_wrapper!(DNS_BASE_DOMAIN, dns_base_domain, String);

pub fn init() {
    dotenv().ok();
    *server_domain().lock().unwrap() = std::env::var("SERVER_DOMAIN")
        .expect("SERVER_DOMAIN must be set.")
        .parse::<String>()
        .expect("SERVER_DOMAIN must be string.");
    *server_port().lock().unwrap() = std::env::var("SERVER_PORT")
        .expect("SERVER_PORT must be set.")
        .parse::<u16>()
        .expect("SERVER_PORT must be integer.");
    *dns_key().lock().unwrap() = std::env::var("DNS_KEY")
        .expect("DNS_KEY must be set.")
        .parse::<String>()
        .expect("SERVER_DOMAIN must be string.");
    *dns_server().lock().unwrap() = std::env::var("DNS_SERVER")
        .expect("DNS_SERVER must be set.")
        .parse::<String>()
        .expect("SERVER_DOMAIN must be string.");
    *dns_base_domain().lock().unwrap() = std::env::var("DNS_BASE_DOMAIN")
        .expect("DNS_BASE_DOMAIN must be set.")
        .parse::<String>()
        .expect("SERVER_DOMAIN must be string.");
}
