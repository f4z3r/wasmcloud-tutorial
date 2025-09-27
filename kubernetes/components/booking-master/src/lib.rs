wit_bindgen::generate!({ generate_all });

use exports::demo::booking_master::booking_management::Guest;
use wasmcloud_component::wasi::logging::logging;

struct Component {}

const LOG_CONTEXT: &'static str = "demo.booking-master";

impl Guest for Component {
    fn add_booking(id: u32, subject: _rt::String) -> Result<(), _rt::String> {
        logging::log(logging::Level::Info, LOG_CONTEXT, &format!("Created booking {}: {}", id, subject));
        Ok(())
    }

    fn get_booking(id: u32) -> Result<_rt::String, _rt::String> {
        logging::log(logging::Level::Info, LOG_CONTEXT, &format!("Returning booking {}", id));
        Ok("some booking".into())
    }

    fn delete_booking(id: u32) -> Result<(), _rt::String> {
        logging::log(logging::Level::Info, LOG_CONTEXT, &format!("Deleting booking {}", id));
        Ok(())
    }
}

export!(Component);
