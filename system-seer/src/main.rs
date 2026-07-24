use axum::{Json, Router, routing::get, extract::{State,Path, Query}};
use tokio::net::TcpListener;
use sysinfo::{
    Components, Disks, Networks, System, Pid
};
use serde::{Deserialize, Serialize};
use std::{result, sync::Arc};

struct AppState {
    sys: System
}

#[derive(Deserialize)]
struct QueryTop {
    n: usize,
}


#[derive(Debug, Serialize, Deserialize)]
struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState{sys: System::new_all()});
    
    let app = Router::new()
        .route("/processes", get(get_processes))
        .route("/processes/{pid}", get(get_process))
        .route("/processes/top", get(get_top_processes))
        .with_state(app_state.clone())
    ;

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn get_processes(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<ProcessInfo>> {
    let processes = state.sys.processes();
    let processes: Vec<ProcessInfo> = processes.iter().map(
        |(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_str().unwrap().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory()
        }
    ).collect();
    Json(processes)
}

async fn get_process(
    Path(pid): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Json<ProcessInfo> {
    let process = state.sys.process(Pid::from_u32(pid)).unwrap();
    
    let process = ProcessInfo{
        pid: pid,
        name: process.name().to_str().unwrap().to_string(),
        cpu_usage: process.cpu_usage(),
        memory_usage: process.memory()
    };
    
    Json(process)
}

async fn get_top_processes(
    Query(n): Query<QueryTop>,
    State(state): State<Arc<AppState>>,
) -> Json<Vec<ProcessInfo>> {
    let processes = state.sys.processes();
    
    let mut result: Vec<ProcessInfo> = processes.iter().map(
        |(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_str().unwrap().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory()
        }
    ).collect();

    result.sort_by_key(|process| std::cmp::Reverse(process.memory_usage));
    Json(result.into_iter().take(n.n).collect())
}
