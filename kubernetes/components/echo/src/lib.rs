wit_bindgen::generate!({ generate_all });

use std::num::ParseIntError;

use anyhow::{anyhow, Result};

use wasi::logging::logging::*;
use wasmcloud::messaging::*;

use wasmcloud_component::http;

use crate::demo::booking_master::booking_management;

const LOG_CONTEXT: &'static str = "demo.echo";
const PUBLISH_SUBJECT: &'static str = "bookings.events";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RequestBody {
    booking: String,
}

struct Component;

impl Component {
    fn parse_id(path: &str) -> Result<u32> {
        path.split("/")
            .last()
            .ok_or(anyhow!("Failed to parse ID from path: {path}"))?
            .parse()
            .map_err(|e: ParseIntError| e.into())
    }
}

impl http::Server for Component {
    fn handle(
        mut request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        match request.method() {
            &http::Method::GET => {
                let path = request.uri().to_string();
                let id = Self::parse_id(&path)
                    .map_err(|e| http::ErrorCode::InternalError(Some(e.to_string())))?;
                let booking = booking_management::get_booking(id)
                    .map_err(|e| http::ErrorCode::InternalError(Some(e.into())))?;
                consumer::publish(&types::BrokerMessage {
                    subject: PUBLISH_SUBJECT.into(),
                    reply_to: None,
                    body: format!("Retrieved booking {id}: {booking}").into(),
                })
                .map_err(|e| http::ErrorCode::InternalError(Some(e.into())))?;
                log(
                    Level::Info,
                    LOG_CONTEXT,
                    &format!("handled GET request for ID {id}"),
                );
                Ok(http::Response::new(format!("Booking {id} created\n")))
            }
            &http::Method::POST => {
                let path = request.uri().to_string();
                let id = Self::parse_id(&path)
                    .map_err(|e| http::ErrorCode::InternalError(Some(e.to_string())))?;
                let subject: RequestBody = match serde_json::from_reader(request.body_mut()) {
                    Ok(v) => v,
                    Err(e) => {
                        return http::Response::builder()
                            .status(http::StatusCode::BAD_REQUEST)
                            .body(format!("invalid booking request: {e}").into())
                            .map_err(|e| http::ErrorCode::InternalError(Some(e.to_string())));
                    }
                };
                let booking = subject.booking;
                booking_management::add_booking(id, &booking)
                    .map_err(|e| http::ErrorCode::InternalError(Some(e.into())))?;
                consumer::publish(&types::BrokerMessage {
                    subject: PUBLISH_SUBJECT.into(),
                    reply_to: None,
                    body: format!("Created booking {id}: {booking}").into(),
                })
                .map_err(|e| http::ErrorCode::InternalError(Some(e.into())))?;
                log(
                    Level::Info,
                    LOG_CONTEXT,
                    &format!("handled POST request for ID {id}"),
                );
                Ok(http::Response::new(format!("Booking {id}: {booking}\n")))
            }
            _ => {
                // TODO(@f4z3r): support delete
                panic!("fail");
            }
        }
    }
}

http::export!(Component);
