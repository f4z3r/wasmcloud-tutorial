wit_bindgen::generate!({ generate_all });

// use wasi::logging::logging::*;
use wasmcloud::messaging::*;

use wasmcloud_component::http;

struct Component;

http::export!(Component);

impl http::Server for Component {
    fn handle(
        _request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        consumer::publish(&types::BrokerMessage {
            subject: "echo.response".into(),
            reply_to: None,
            body: "This is a test".into(),
        }).map_err(|e| http::ErrorCode::InternalError(Some(e.into())))?;

        Ok(http::Response::new("Published a message to echo.response subject\n"))
    }
}

    // fn handle_message(msg: types::BrokerMessage) -> Result<(), String> {
    //     if let Some(reply_to) = msg.reply_to {
    //         consumer::publish(&types::BrokerMessage {
    //             subject: reply_to,
    //             reply_to: None,
    //             body: msg.body,
    //         })
    //     } else {
    //         log(
    //             Level::Warn,
    //             "",
    //             "No reply_to field in message, ignoring message",
    //         );
    //         Ok(())
    //     }
    // }
