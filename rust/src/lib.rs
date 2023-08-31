use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    http_component, http_router,
    key_value::{Error as KeyValueError, Store},
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_rust(req: Request) -> Result<Response> {
    let router = http_router! {
        GET "/rust/:id"=> get_stuff,
        POST "/rust/:id" => set_stuff,
        _ "/*" => |_req, _params| {
            Ok(http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(None)
                .unwrap())
        }
    };

    router.handle(req)
}

fn set_stuff(req: Request, params: Params) -> Result<Response> {
    let store = Store::open_default()?;

    let id = params.get("id").unwrap();

    match store.exists(id) {
        Ok(true) => println!("Updating key {} in the KV Store", id),
        Ok(false) => println!("Storing key {} in the KV Store", id),
        Err(error) => println!("Help!!! {}", error),
    };

    store.set(id, req.body().as_deref().unwrap_or(&[]))?;

    return Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(None)?);
}

fn get_stuff(_req: Request, params: Params) -> Result<Response> {
    let store = Store::open_default()?;

    let id = params.get("id").unwrap();

    let (body, code) = match store.get(id) {
        Ok(value) => (value.to_vec(), http::StatusCode::OK),
        Err(KeyValueError::NoSuchKey) => (
            "Key not found".as_bytes().to_vec(),
            http::StatusCode::NOT_FOUND,
        ),
        Err(error) => (
            format!("{}", error).as_bytes().to_vec(),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    };

    return Ok(http::Response::builder()
        .status(code)
        .body(Some(body.into()))?);
}
