use wasmcloud_component::http::ErrorCode;
use wasmcloud_component::wasi::keyvalue::*;
use wasmcloud_component::{http, info};

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
        let name = match query.split("=").collect::<Vec<&str>>()[..] {
            ["name", name] => name,
            _ => "World",
        };

        info!("Greeting {name}");

        let bucket = store::open("default").map_err(|e| {
            ErrorCode::InternalError(Some(format!("failed to open KV bucket: {e:?}")))
        })?;
        let count = atomics::increment(&bucket, &name, 1).map_err(|e| {
            ErrorCode::InternalError(Some(format!("failed to increment counter: {e:?}")))
        })?;

        Ok(http::Response::new(format!("Hello x{count}, {name}!\n")))}
}
