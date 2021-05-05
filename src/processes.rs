use serde::{Deserialize, Serialize};
use actix_web::{web, get, post, HttpResponse};

#[derive(Debug, Deserialize, Serialize)]
pub enum ProcessStatus {
    Good,
    Warning,
    Error,
    MissedHeartbeat,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Process {
    pub id: String,
    pub status: ProcessStatus,
}

impl Process {
    pub fn new(id: String) -> Self {
        Self {
            id: id,
            status: ProcessStatus::Unknown,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessStartRequest {
    pub id: String,
}

impl ProcessStartRequest {
    pub fn to_process(&self) -> Process {
        Process::new(self.id.clone())
    }
}

/// list running processes
#[get("/processes")]
pub async fn list() -> HttpResponse {
    // TODO list processes
    // TODO even later... pagination

    let processes: Vec<Process> = vec![];

    HttpResponse::Ok()
        .content_type("application/json")
        .json(processes)
}

/// start a process
#[post("/processes")]
pub async fn start(process_req: web::Json<ProcessStartRequest>) -> HttpResponse {
    // TODO start process
    HttpResponse::Created()
        .content_type("application/json")
        .json(process_req.to_process())
}

/// find a process by its id `/processes/{id}`
#[get("/processes/{id}")]
pub async fn get(path: web::Path<(String,)>) -> HttpResponse {
    let id = path.0;
    // TODO find process by ID and return it
    let found_process: Option<Process> = None;

    match found_process {
        Some(process) => HttpResponse::Ok()
            .content_type("application/json")
            .json(process),
        None => HttpResponse::NoContent()
            .content_type("application/json")
            .await
            .unwrap(),
    }
}
