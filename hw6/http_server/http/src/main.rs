use std::net::SocketAddr;
use mini_redis::DEFAULT_ADDR;
use std::sync::Arc;
use axum:: {
    Router,
    routing::get,
    http::StatusCode,
    response::{Response, Html, IntoResponse},
    extract::{Path, State},
    Form,
};

use serde::Deserialize;

use volo_gen::volo::example::{ItemServiceClient, ItemServiceClientBuilder, GetItemRequest, GetItemResponse};

type RpcClient = ItemServiceClient;
type RpcClientBuilder = ItemServiceClientBuilder;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = DEFAULT_ADDR.parse().unwrap();
    let rpc_cli = Arc::new(RpcClientBuilder::new("rpc_mini_redis").address(addr).build());
    // build the application with router
    let app = Router::new()
        .route("/ping", get(ping)).with_state(rpc_cli.clone())
        .route("/get/:keys", get(get_key).with_state(rpc_cli.clone()))
        .route(
            "/set",
            get(show_set_key_form).post(set_key).with_state(rpc_cli.clone()),
        )
        .route("/del", get(show_del_form).post(del_key).with_state(rpc_cli.clone()))
        .route("/subscribe", get(show_channel_form).post(subscribe).with_state(rpc_cli.clone()))
        .route("/publish", get(show_channel_value_form).post(publish).with_state(rpc_cli.clone()))
        .route("/ping/:message", get(ping_with_message).with_state(rpc_cli.clone()));
    
    // run it with hyper
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = "[::]:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// The function of ping
async fn ping(State(rpc_cli): State<Arc<RpcClient>>) -> (StatusCode, String) {
    let req = GetItemRequest {
        opcode: 3,
        key_channal: " ".into(),
        value_message: " ".into(),
    };
    let resp = rpc_cli.get_item(req).await;
    let mut _resp_final: GetItemResponse = GetItemResponse {
        opcode: 0,
        key_channal: " ".into(),
        value_message: " ".into(),
        success: false
    };
    match resp {
        Ok(r) => {
            _resp_final = r;
        },
        Err(e) => {
            return (StatusCode::OK, format!("{}", e));
        },
    }
    if _resp_final.success {
        (StatusCode::OK, "pong".to_string())
    } else {
        (StatusCode::OK, "The connect is fail".to_string())
    }
}

// The function of ping with message
async fn ping_with_message(Path(message): Path<String>, State(rpc_cli): State<Arc<RpcClient>>) -> Response {
    let req = GetItemRequest {
        opcode: 3,
        key_channal: " ".into(),
        value_message: message.clone().into(),
    };
    let resp = rpc_cli.get_item(req).await;
    let mut _resp_final: GetItemResponse = GetItemResponse {
        opcode: 0,
        key_channal: " ".into(),
        value_message: " ".into(),
        success: false
    };
    match resp {
        Ok(r) => {
            _resp_final = r;
        },
        Err(e) => {
            return (StatusCode::OK, format!("{}", e)).into_response();
        },
    }
    if _resp_final.success && _resp_final.value_message.to_string().eq(&message) {
        (StatusCode::OK, _resp_final.value_message.to_string()).into_response()
    } else {
        (StatusCode::OK, "The connect is fail".to_string()).into_response()
    }
}

// The function of get key
async fn get_key(Path(key): Path<String>, State(rpc_cli): State<Arc<RpcClient>>) -> Response {
    let req = GetItemRequest {
        opcode: 0,
        key_channal: key.into(),
        value_message: " ".into(),
    };
    let resp = rpc_cli.get_item(req).await;
    let mut _resp_final: GetItemResponse = GetItemResponse {
        opcode: 0,
        key_channal: " ".into(),
        value_message: " ".into(),
        success: false
    };
    match resp {
        Ok(r) => {
            _resp_final = r;
        },
        Err(e) => {
            return (StatusCode::OK, e.to_string()).into_response();
        },
    }
    if _resp_final.success {
        (StatusCode::OK, _resp_final.value_message.to_string()).into_response()
    } else {
        (StatusCode::OK, format!("The key: {} is not in the database", _resp_final.key_channal)).into_response()
    }
}

#[derive(Deserialize, Debug)]
struct FormSetKey {
    key: String,
    val: String,
}

/// Show the form for set a key
async fn show_set_key_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/set" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <label for="val">
                        Enter value:
                        <input type="text" name="val">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}


// The function of set key
async fn set_key(State(rpc_cli): State<Arc<RpcClient>>, Form(setkey): Form<FormSetKey>) -> Response {
    let req = GetItemRequest {
        opcode: 1,
        key_channal: setkey.key.into(),
        value_message: setkey.val.into(),
    };
    let resp = rpc_cli.get_item(req).await;
    let mut _resp_final: GetItemResponse = GetItemResponse {
        opcode: 0,
        key_channal: " ".into(),
        value_message: " ".into(),
        success: false
    };
    match resp {
        Ok(r) => {
            _resp_final = r;
        },
        Err(e) => {
            return (StatusCode::OK, e.to_string()).into_response();
        },
    }
    if _resp_final.success {
        (StatusCode::OK, "Set success!").into_response()
    } else {
        (StatusCode::OK, format!("The key: {} is already in the database", _resp_final.key_channal)).into_response()
    }
}

async fn show_del_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/del" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
struct FormKey {
    key: String,
}

// Delete a kee value pair from redis
async fn del_key(
    State(rpc_cli): State<Arc<RpcClient>>,
    Form(delkey): Form<FormKey>,
) -> (StatusCode, String) {
    let req = GetItemRequest {
        opcode: 2,
        key_channal: delkey.key.into(),
        value_message: " ".into(),
    };
    let resp = rpc_cli.get_item(req).await;
    let mut _resp_final: GetItemResponse = GetItemResponse {
        opcode: 0,
        key_channal: " ".into(),
        value_message: " ".into(),
        success: false
    };
    match resp {
        Ok(r) => {
            _resp_final = r;
        },
        Err(e) => {
            return (StatusCode::OK, e.to_string());
        },
    }
    if _resp_final.success {
        (StatusCode::OK, "Del success!".to_string())
    } else {
        (StatusCode::OK, format!("The key: {} is not found in the database", _resp_final.key_channal))
    }
}


#[derive(Deserialize, Debug)]
struct FormChannel {
    channel_name: String,
}

async fn show_channel_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/subscribe" method="post">
                    <label for="channel_name">
                        Enter channel_name:
                        <input type="text" name="channel_name">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

// Subscribe a channel
async fn subscribe(State(rpc_cli): State<Arc<RpcClient>>, Form(channel): Form<FormChannel> ) -> (StatusCode, String) {
    let req = GetItemRequest {
        opcode: 4,
        key_channal: channel.channel_name.into(),
        value_message: " ".into(),
    };
    let resp = rpc_cli.get_item(req).await;
    let mut _resp_final: GetItemResponse = GetItemResponse {
        opcode: 0,
        key_channal: " ".into(),
        value_message: " ".into(),
        success: false
    };
    match resp {
        Ok(r) => {
            _resp_final = r;
        },
        Err(e) => {
            return (StatusCode::OK, e.to_string());
        },
    }
    if _resp_final.success {
        (StatusCode::OK, _resp_final.value_message.clone().to_string())
    } else {
        (StatusCode::OK, "no publish".to_string())
    }
}

#[derive(Deserialize, Debug)]
struct FormChannelValue {
    channel_name: String,
    value: String
}

async fn show_channel_value_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/publish" method="post">
                    <label for="channel_name">
                        Enter channel_name:
                        <input type="text" name="channel_name">
                    </label>
                    <label for="value">
                        Enter value:
                        <input type="text" name="value">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

// publish a message to a channel
async fn publish(State(rpc_cli): State<Arc<RpcClient>>, Form(channel): Form<FormChannelValue> ) -> (StatusCode, String) {
    let req = GetItemRequest {
        opcode: 5,
        key_channal: channel.channel_name.into(),
        value_message: channel.value.into(),
    };
    let resp = rpc_cli.get_item(req).await;
    let mut _resp_final: GetItemResponse = GetItemResponse {
        opcode: 0,
        key_channal: " ".into(),
        value_message: " ".into(),
        success: false
    };
    match resp {
        Ok(r) => {
            _resp_final = r;
        },
        Err(e) => {
            return (StatusCode::OK, e.to_string());
        },
    }
    let message: String = _resp_final.value_message.clone().into();
    let v : Vec<char> = message.chars().collect();
    if _resp_final.success {
        (StatusCode::OK, format!("publish success. The number of subscriber is {}", get_num(&v)))
    } else {
        (StatusCode::OK, "No subscriber found".to_string())
    }
}

fn get_num(v: &Vec<char>) -> i32 {
    let mut index = 0;
    let mut res = 0;
    while index < v.len() {
        res = res * 10 + (v[index] as i32 - '0' as i32);
        index += 1;
    }
    res
}

