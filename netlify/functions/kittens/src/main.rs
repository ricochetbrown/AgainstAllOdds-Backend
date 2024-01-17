use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use actix_web::{App, HttpServer};
use std::any::Any;

use cats::{create_cat_data, create_cat_scope};

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: ApiGatewayProxyRequest, _ctx: Context) -> Result<dyn Any, Error> {
    let path = event.path.unwrap();
    let cat_data = create_cat_data();

    let resp: dyn Any = HttpServer::new(move || App::new().service(create_cat_scope(&cat_data))).bind(path) as Any;

    Ok(resp);
}