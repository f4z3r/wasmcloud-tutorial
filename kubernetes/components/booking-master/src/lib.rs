wit_bindgen::generate!({ generate_all });

use exports::demo::booking_master::booking_management::Guest;
use wasi::keyvalue::store;
use wasi::logging::logging;

struct Component {}

const LOG_CONTEXT: &'static str = "demo.booking-master";

impl Component {
    fn bucket() -> Result<store::Bucket, _rt::String> {
        // Redis does not support bucket names
        store::open("").map_err(|e| format!("failed to open bucket: {e:?}"))
    }
}

impl Guest for Component {
    fn add_booking(id: u32, subject: _rt::String) -> Result<(), _rt::String> {
        logging::log(
            logging::Level::Info,
            LOG_CONTEXT,
            &format!("Created booking {}: {}", id, subject),
        );
        Self::bucket()?
            .set(&id.to_string(), subject.as_bytes())
            .map_err(|e| format!("failed to store booking: {e:?}"))
    }

    fn get_booking(id: u32) -> Result<_rt::String, _rt::String> {
        logging::log(
            logging::Level::Info,
            LOG_CONTEXT,
            &format!("Returning booking {}", id),
        );
        let val = Self::bucket()?
            .get(&id.to_string())
            .map_err(|e| format!("failed to store booking: {e:?}"))?;
        if let Some(slice) = val {
            String::from_utf8(slice)
                .map_err(|e| format!("failed to convert booking to string: {e:?}"))
        } else {
            Err(format!("no booking found for id: {id}"))
        }
    }

    fn delete_booking(id: u32) -> Result<(), _rt::String> {
        logging::log(
            logging::Level::Info,
            LOG_CONTEXT,
            &format!("Deleting booking {}", id),
        );
        Self::bucket()?
            .delete(&id.to_string())
            .map_err(|e| format!("failed to delete booking: {e:?}"))
    }
}

export!(Component);
