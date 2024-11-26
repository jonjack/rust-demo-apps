use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct Response {
    payload: Value,
}

async fn handler(event: LambdaEvent<Value>) -> Result<Response, Error> {
    let payload = event.payload;
    tracing::info!("Payload {}.", payload);
    let resp = Response { payload: payload };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(handler)).await
}
