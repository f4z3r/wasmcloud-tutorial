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
                // TODO: use the `store::set` method to store the key and value using the provider.
                (
                     http::StatusCode::CREATED,
                    // TODO: and a message that tells the client that the store action was
                    // successful. You can use the `format!` macro for this.
                )
            }
            // Case 2: Query does not contain '=', implying GET operation or Welcome message
            None => {
                let key = query.trim();

                if key.is_empty() {
                    (
                        // TODO: the key is empty, return a "BAD_REQUEST" error code. See the
                        // CREATED error code above for reference.
                        ,
                        // TODO: return a meaningful error message to tell the user how to use the
                        // API.
                    )
                } else {
                    match store::get(key) {
                        // Success: Value found
                        Some(value) => (
                            // TODO: the key is empty, return a "OK" error code. See the
                            // CREATED error code above for reference
                            ,
                            // TODO: return a message to the client telling it what value was
                            // stored for the requested key. You can use the `format!` macro for
                            // this.
                        ),
                        // Success: Key not found
                        None => (
                            // TODO: the key is empty, return a "NOT_FOUND" error code. See the
                            // CREATED error code above for reference.
                            ,
                            // TODO: return a message to the client telling it that no value was
                            // found for the requested key.
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
