use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::CONTENT_TYPE;
use http::HeaderMap;
use lambda_runtime::{Context, Error, handler_fn};
use serde::Serialize;
use tracing::{debug, info, instrument};

#[derive(Serialize)]
struct ApiResponse {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    debug!("cold boot");

    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[instrument]
pub(crate) async fn my_handler(event: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, Error> {
    let who = event
        .query_string_parameters.get("name");
    info!(who, "query accepted");


    let message = format!(
        "Hello {who}, this is an Netlify serverless request"
    );
    let api_response = ApiResponse { data: message };
    let body_text = serde_json::to_string(&api_response)?;

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(body_text)),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}