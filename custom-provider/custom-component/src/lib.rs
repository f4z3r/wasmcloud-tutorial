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
        
        let mut status = http::StatusCode::OK;
        
        // Determine the action (SET or GET) based on the query string
        let response_body = match query.split_once('=') {
            // Case 1: Query contains '=', implying SET operation (e.g., ?key=value)
            Some((key, value)) => {
                if let Err(_error) = store::set(&key, &value) {
                    return Err(ErrorCode::InternalError(Some(format!(
                        "failed to write key!"
                    ))));
                }
                format!("{key} added with value: {value}!\n")

            }
            // Case 2: Query does not contain '=', implying GET operation or Welcome message
            None => {
                let key = query.trim();
                
                if key.is_empty() {
                    format!("Use the query string: ?key=value (SET) or ?key (GET).")
                } else {
                    match store::get(key) {
                        // Success: Value found
                        Ok(Some(value)) => format!("Value for '{key}': {value}"),
                        // Success: Key not found
                        Ok(None) => {
                            status = http::StatusCode::NOT_FOUND;
                            format!("Key '{key}' not found.")
                        },
                        // Failure: Underlying store error
                        Err(e) => {
                            return Err(ErrorCode::InternalError(Some(format!("Store get failed: {:?}", e))));
                        }
                    }
                }
            }
        };

        let response = http::Response::builder()
            .status(status)
            .body(response_body);
        
        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                eprintln!("Error building response: {}", e);
                Err(ErrorCode::InternalError(Some("Failed to finalize HTTP response".to_string())))
            }
        }
    }
}
