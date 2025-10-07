// wasmCloud KV Store Component (Minimal Tutorial Version)

// wit_bindgen::generate!({ generate_all });
mod bindings;
use bindings::wasmcloud_tutorial::key_value_provider::store;
use wasmcloud_component::http;
use wasmcloud_component::http::ErrorCode; // Used for converting store errors to HTTP errors

struct CustomComponent;

http::export!(CustomComponent);

impl http::Server for CustomComponent {
    fn handle(
        request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        // Consume the request to get parts (headers, URI)
        let (parts, _body) = request.into_parts();

        // Extract the query string from the URI
        let query = parts
            .uri
            .query()
            .map(ToString::to_string)
            .unwrap_or_default();

        // Determine the action (SET or GET) based on the query string
        let (status, response_body) = match query.split_once('=') {
            // Case 1: Query contains '=', implying SET operation (e.g., ?key=value)
            Some((key, value)) => {
                store::set(&key, &value);
                (
                    http::StatusCode::CREATED,
                    format!("{key} added with value: {value}!\n"),
                )
            }
            // Case 2: Query does not contain '=', implying GET operation or Welcome message
            None => {
                let key = query.trim();

                if key.is_empty() {
                    (
                        http::StatusCode::BAD_REQUEST,
                        format!("Use the query string: ?key=value (SET) or ?key (GET)."),
                    )
                } else {
                    match store::get(key) {
                        // Success: Value found
                        Some(value) => {
                            (http::StatusCode::OK, format!("Value for '{key}': {value}"))
                        }
                        // Success: Key not found
                        None => (
                            http::StatusCode::NOT_FOUND,
                            format!("Key '{key}' not found."),
                        ),
                    }
                }
            }
        };

        let response = http::Response::builder().status(status).body(response_body);

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                eprintln!("Error building response: {}", e);
                Err(ErrorCode::InternalError(Some(
                    "Failed to finalize HTTP response".to_string(),
                )))
            }
        }
    }
}
