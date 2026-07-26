use axum::{extract::ws::{Message, WebSocket, WebSocketUpgrade}, extract::Query, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, path::PathBuf, process::Stdio, sync::Arc};
use tokio::{fs, io::{AsyncBufReadExt, BufReader}, process::{Child, Command}, sync::{broadcast, Mutex}};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateRequest {
    name: String,
    engine: String,
    version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IdRequest {
    id: String,
}

#[derive(Debug)]
struct ServerInstance {
    id: String,
    name: String,
    path: PathBuf,
    process: Option<Child>,
    broadcaster: broadcast::Sender<String>,
}

type SharedState = Arc<Mutex<HashMap<String, ServerInstance>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state: SharedState = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/create", post(create_server))
        .route("/start", post(start_server))
        .route("/stop", post(stop_server))
        .route("/status", get(status))
        .route("/ws/logs", get(ws_logs))
        .layer(axum::Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    println!("Runtime manager listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "MineServers runtime manager"
}

async fn create_server(axum::Extension(state): axum::Extension<SharedState>, Json(payload): Json<CreateRequest>) -> axum::Json<serde_json::Value> {
    let id = Uuid::new_v4().to_string();
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("./data"));
    let servers_dir = base.join("mineservers").join("servers");
    let server_dir = servers_dir.join(&id);
    fs::create_dir_all(&server_dir).await.unwrap();

    // download paper jar if engine == paper
    if payload.engine == "paper" {
        // Determine version
        let version = if payload.version == "latest" {
            // fetch latest version from PaperMC API
            match reqwest::get("https://api.papermc.io/v2/projects/paper").await {
                Ok(resp) => {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(versions) = json.get("versions").and_then(|v| v.as_array()) {
                            versions.last().and_then(|v| v.as_str()).unwrap_or("1.20.2").to_string()
                        } else {
                            "1.20.2".to_string()
                        }
                    } else {
                        "1.20.2".to_string()
                    }
                }
                Err(_) => "1.20.2".to_string(),
            }
        } else {
            payload.version.clone()
        };

        // get latest build for version
        let build_info_url = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/latest", version);
        let mut build_number = None;
        if let Ok(resp) = reqwest::get(&build_info_url).await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                build_number = json.get("build")
                    .and_then(|b| b.as_u64());
            }
        }

        if let Some(build) = build_number {
            let jar_url = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/paper-{}-{}.jar", version, build, version, build);
            let jar_path = server_dir.join("paper.jar");
            // download jar
            match reqwest::get(&jar_url).await {
                Ok(resp) => {
                    let bytes = resp.bytes().await.unwrap_or_default();
                    if fs::write(&jar_path, &bytes).await.is_err() {
                        println!("Failed to write jar to {:?}", jar_path);
                    }
                }
                Err(e) => println!("Failed to download jar: {}", e),
            }
        } else {
            println!("Could not determine build number for version {}", version);
        }
    }

    // write a default eula.txt (user must accept in UI to enable public access)
    let eula_path = server_dir.join("eula.txt");
    let _ = fs::write(&eula_path, "eula=true\n").await;

    let (tx, _rx) = broadcast::channel(1024);

    let instance = ServerInstance {
        id: id.clone(),
        name: payload.name.clone(),
        path: server_dir.clone(),
        process: None,
        broadcaster: tx,
    };

    state.lock().await.insert(id.clone(), instance);

    axum::Json(serde_json::json!({"id": id, "path": server_dir.to_string_lossy()}))
}

async fn start_server(axum::Extension(state): axum::Extension<SharedState>, Json(payload): Json<IdRequest>) -> axum::Json<serde_json::Value> {
    let mut map = state.lock().await;
    if let Some(inst) = map.get_mut(&payload.id) {
        if inst.process.is_some() {
            return axum::Json(serde_json::json!({"status": "already_running"}));
        }
        // spawn java process
        let jar = inst.path.join("paper.jar");
        let mut cmd = Command::new("java");
        cmd.arg("-Xmx1G").arg("-jar").arg(jar).arg("nogui");
        cmd.current_dir(&inst.path);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                // setup log forwarding
                let mut stdout = child.stdout.take().map(BufReader::new);
                let mut stderr = child.stderr.take().map(BufReader::new);
                let tx = inst.broadcaster.clone();
                let id = inst.id.clone();

                // spawn task for stdout
                if let Some(mut out) = stdout {
                    let mut lines = out.lines();
                    let tx2 = tx.clone();
                    tokio::spawn(async move {
                        while let Ok(Some(line)) = lines.next_line().await {
                            let _ = tx2.send(format!("[{}] {}", id, line));
                        }
                    });
                }

                if let Some(mut err) = stderr {
                    let mut lines = err.lines();
                    let tx2 = tx.clone();
                    tokio::spawn(async move {
                        while let Ok(Some(line)) = lines.next_line().await {
                            let _ = tx2.send(format!("[{}][ERR] {}", id, line));
                        }
                    });
                }

                inst.process = Some(child);
                return axum::Json(serde_json::json!({"status": "started"}));
            }
            Err(e) => {
                return axum::Json(serde_json::json!({"status": "error", "error": e.to_string()}));
            }
        }
    }
    axum::Json(serde_json::json!({"status": "not_found"}))
}

async fn stop_server(axum::Extension(state): axum::Extension<SharedState>, Json(payload): Json<IdRequest>) -> axum::Json<serde_json::Value> {
    let mut map = state.lock().await;
    if let Some(inst) = map.get_mut(&payload.id) {
        if let Some(mut child) = inst.process.take() {
            let _ = child.kill().await;
            return axum::Json(serde_json::json!({"status": "stopped"}));
        } else {
            return axum::Json(serde_json::json!({"status": "not_running"}));
        }
    }
    axum::Json(serde_json::json!({"status": "not_found"}))
}

async fn status(Query(params): Query<HashMap<String, String>>, axum::Extension(state): axum::Extension<SharedState>) -> axum::Json<serde_json::Value> {
    if let Some(id) = params.get("id") {
        let map = state.lock().await;
        if let Some(inst) = map.get(id) {
            let running = inst.process.is_some();
            return axum::Json(serde_json::json!({"id": id, "running": running, "path": inst.path.to_string_lossy()}));
        }
    }
    axum::Json(serde_json::json!({"status": "not_found"}))
}

async fn ws_logs(Query(params): Query<HashMap<String, String>>, ws: WebSocketUpgrade, axum::Extension(state): axum::Extension<SharedState>) -> impl axum::response::IntoResponse {
    let id = params.get("id").cloned().unwrap_or_default();
    ws.on_upgrade(move |socket| handle_ws(socket, id, state))
}

async fn handle_ws(mut socket: WebSocket, id: String, state: SharedState) {
    // subscribe to broadcaster
    let rx_opt = {
        let map = state.lock().await;
        map.get(&id).map(|inst| inst.broadcaster.subscribe())
    };
    if rx_opt.is_none() {
        let _ = socket.send(Message::Text(format!("no server with id {}", id))).await;
        let _ = socket.close().await;
        return;
    }
    let mut rx = rx_opt.unwrap();

    // spawn a task to forward broadcast messages to websocket
    let mut ws_send = socket.clone();
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    let _ = ws_send.send(Message::Text(msg)).await;
                }
                Err(_) => break,
            }
        }
    });

    // also read messages from client (console input)
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                // treat text as console input: write to process stdin if available
                let mut map = state.lock().await;
                if let Some(inst) = map.get_mut(&id) {
                    if let Some(child) = inst.process.as_mut() {
                        if let Some(mut stdin) = child.stdin.take() {
                            use tokio::io::AsyncWriteExt;
                            let _ = stdin.write_all((text + "\n").as_bytes()).await;
                            let _ = child.stdin.replace(stdin);
                        }
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}
