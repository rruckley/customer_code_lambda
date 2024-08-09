use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use tmflib::gen_code;


#[derive(Deserialize)]
struct CodeRequest {
    id: String,
    name: String,
    offset: Option<u32>,
    prefix: Option<String>,
    length: Option<usize>,
}

#[derive(Default,Serialize)]
struct CodeResponse {
    code: String,
    hash: String,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    match event.body() {
        Body::Text(t) => {
            // Process String body
            let code_request : CodeRequest = serde_json::from_str(t.as_str()).unwrap();
            let (code,hash) = gen_code(code_request.name, code_request.id, code_request.offset, code_request.prefix, code_request.length);
            let code_response = CodeResponse {
                code,
                hash,
            };
            let body = Body::from_maybe_encoded(false, serde_json::to_string(&code_response).unwrap().as_str());
            let resp = Response::builder()
                .status(200)
                .header("conent-type", "applicaiton/json")
                .body(body)
                .map_err(Box::new)?;
            Ok(resp)
        },
        Body::Empty => {
            let body = Body::from_maybe_encoded(false, serde_json::to_string(&CodeResponse::default()).unwrap().as_str());
            let resp = Response::builder()
                .status(201)
                .body(body)
                .map_err(Box::new)?;
            Ok(resp)
        },
        Body::Binary(_b) => {
            let body = Body::from_maybe_encoded(false, serde_json::to_string(&CodeResponse::default()).unwrap().as_str());
            let resp = Response::builder()
            .status(201)
            .body(body)
            .map_err(Box::new)?;
             Ok(resp)    
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
