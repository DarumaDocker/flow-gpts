use flowsnet_platform_sdk::logger;
use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{
    create_endpoint, request_handler,
    route::{post, route, RouteError, Router},
    send_response,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler() {
    logger::init();
    let mut router = Router::new();
    router.insert("/oauth", vec![post(oauth)]).unwrap();

    if let Err(e) = route(router).await {
        match e {
            RouteError::NotFound => {
                send_response(404, vec![], b"No route matched".to_vec());
            }
            RouteError::MethodNotAllowed => {
                send_response(405, vec![], b"Method not allowed".to_vec());
            }
        }
    }
}

async fn oauth(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>) {
    send_response(200, vec![], b"OK".to_vec());
}
