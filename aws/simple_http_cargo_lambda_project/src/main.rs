use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    payload: Value,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let parsed: Result<Payload, _> = serde_json::from_slice(event.body());

    match parsed {
        Ok(event) => {
            tracing::info!("Payload: {}", event.payload);
            let response_body = json!({ "payload": event.payload, });
            Ok(Response::new(Body::Text(response_body.to_string())))
        }

        Err(e) => {
            let error_message = json!({
                "error": "Bad Request"
            });

            Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(Body::Text(error_message.to_string()))
                .expect("Failed to render response"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
