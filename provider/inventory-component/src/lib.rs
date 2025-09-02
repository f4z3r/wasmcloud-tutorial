use wasmcloud_component::http;
use wasmcloud_component::http::ErrorCode;
use wasmcloud_component::wasi::keyvalue::*;

struct Component;

http::export!(Component);

impl http::Server for Component {
    fn handle(
        request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let (parts, _body) = request.into_parts();
        let query = parts
            .uri
            .query()
            .map(ToString::to_string)
            .unwrap_or_default();
        let item = match query.split("=").collect::<Vec<&str>>()[..] {
            ["item", item] => item,
            _ => "Missed",
        };
        let bucket = store::open("default").map_err(|e| {
            ErrorCode::InternalError(Some(format!("failed to open KV bucket: {e:?}")))
        })?;
        let count = atomics::increment(&bucket, &item, 1).map_err(|e| {
            ErrorCode::InternalError(Some(format!("failed to increment counter: {e:?}")))
        })?;
        Ok(http::Response::new(format!(
            "{item} added x{count} times to inventory!\n"
        )))
    }
}
