//! The Altar Gateway
//! HTTP/WebSocket API layer for external esoteric applications to interface with the OVM.

use axum::{
    routing::{get, post},
    Router,
    Json,
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ovm::OVM;
use crate::execution::AppManifestExecutor;

#[derive(Deserialize)]
pub struct IntentPayload {
    script_payload: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    resonance: f64,
}

#[derive(Serialize)]
pub struct IntentResponse {
    status: String,
    message: String,
    visual_sigil: Option<Vec<crate::sigil::SigilPoint>>,
    sonic_parameters: Option<Vec<f32>>,
    generation_version: usize,
}

/// Spawns the Axum API gateway for the Occult Virtual Machine.
pub async fn start_gateway() {
    let rate_limiter = Arc::new(AtomicUsize::new(0));

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/cast_intent", post({
            let limiter = Arc::clone(&rate_limiter);
            move |headers: HeaderMap, payload: Json<IntentPayload>| cast_intent(headers, payload, limiter)
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3690));
    println!("[ALTAR_GATEWAY] Constructing API bridge. Binding to {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind gateway to loopback network");
    axum::serve(listener, app).await.expect("Axum gateway critical fault");
}

/// The system vital pulse check.
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OVM Online - Neural Link Active".to_string(),
        resonance: 432.0,
    })
}

/// Ingests arbitrary MagickScript payload from the Altar Gateway and forces execution.
async fn cast_intent(
    headers: HeaderMap, 
    Json(payload): Json<IntentPayload>,
    rate_limiter: Arc<AtomicUsize>
) -> Result<Json<IntentResponse>, (StatusCode, &'static str)> {
    println!("[ALTAR_GATEWAY] Neural Link Active: Evaluating inbound intent...");

    match headers.get("X-MPL-SIGIL") {
        Some(sigil) if sigil == "369-TESLA-RESONANCE" => {}
        _ => {
            println!("[SECURITY_FAULT] Unauthorized manifestation attempt rejected.");
            return Err((StatusCode::UNAUTHORIZED, "Invalid or missing X-MPL-SIGIL."));
        }
    }

    let requests = rate_limiter.fetch_add(1, Ordering::SeqCst);
    if requests > 50 {
        return Err((StatusCode::TOO_MANY_REQUESTS, "Intent flooding detected. Neural pathway saturated."));
    }

    println!("[ALTAR_GATEWAY] Received esoteric schema payload. Commencing synthesis...");

    let lexer = Lexer::new(&payload.script_payload);
    
    println!("[ALTAR_GATEWAY] AST mapped. Spawning ephemeral Virtual Machine instance...");

    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // The runtime instantiation relies on the multi-purpose application executor
    // This allows web or native frontends to execute digital manifestation.
    let mut ovm = OVM::new(432.0, Box::new(AppManifestExecutor::new()));
    ovm.execute(program);

    println!("[ALTAR_GATEWAY] State unified. Returning closure block to client.");

    let visual_sigil = ovm.last_visual_sigil.clone();
    let generation_version = ovm.evolution_engine.generation;

    // Default resonance output for the frontend audio
    let sonic_parameters = Some(crate::stdlib::invoke_sonic_transmutation(ovm.akashic_record.get_temporal_success_rate(), ovm.hz_alignment));

    Ok(Json(IntentResponse {
        status: "Synchronized".to_string(),
        message: "Vector sequence parsed and executed by the Occult Virtual Engine.".to_string(),
        visual_sigil,
        sonic_parameters,
        generation_version,
    }))
}
