wit_bindgen::generate!({ generate_all });

// use wasi::logging::logging::*;
use wasmcloud::messaging::*;

use wasmcloud_component::http;

use crate::demo::booking_master::booking_management;

struct Component;

http::export!(Component);

impl http::Server for Component {
    fn handle(
        _request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let booking = booking_management::get_booking(1).unwrap();
        consumer::publish(&types::BrokerMessage {
            subject: "echo.response".into(),
            reply_to: None,
            body: booking.into(),
        })
        .map_err(|e| http::ErrorCode::InternalError(Some(e.into())))?;

        Ok(http::Response::new(
            "Published a message to echo.response subject\n",
        ))
    }
}
