use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

/*
    This function expects a JSON payload from Amazon API Gateway,
    Amazon Elastic Load Balancer, or a AWS Lambda Function URL.

    Note: You would typically want to use the Serde libraries to deserialize the
    inbound JSON payload into 

    If using Cargo Lambda then:-
    cargo lambda invoke --data-example apigw-request

    See the sample Lambda evernts repository
    https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/lambda-events/src/fixtures/example-apigw-request.json
*/
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body();
    tracing::info!("APIGW Context: {:#?}", event.request_context());
    tracing::info!("Lambda Context: {:#?}", event.lambda_context());

    match body {
        Body::Text(text_body) => {
            tracing::info!("Request body: {}", text_body);
            return Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::Text(text_body.to_owned()))
                .unwrap_or_default());
        }
        Body::Binary(_) => {
            tracing::info!("Request body binary");
            return Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::Text("Binary request type".to_owned()))
                .unwrap_or_default());
        }
        Body::Empty => {
            tracing::info!("Request body empty!");
            return Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::Text("Empty request!".to_owned()))
                .unwrap_or_default());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
