//! The Altar Gateway
//! HTTP/WebSocket API layer for external esoteric applications to interface with the OVM.

use axum::{
    extract::State,
    routing::{get, post},
    Router,
    Json,
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ovm::OVM;
use crate::execution::AppManifestExecutor;

#[derive(Serialize, Clone)]
pub struct SentinelLog {
    pub timestamp: u64,
    pub resonance_score: f64,
    pub intent: String,
    pub seal_id: Option<String>,
}

#[derive(Clone)]
pub struct SharedSentinelState {
    pub logs: Arc<Mutex<Vec<SentinelLog>>>,
}

impl SharedSentinelState {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn push(&self, log: SentinelLog) {
        if let Ok(mut un) = self.logs.lock() {
            un.insert(0, log);
            un.truncate(10);
        }
    }
}

#[derive(Clone)]
struct AppState {
    rate_limiter: Arc<AtomicUsize>,
    sentinel_state: SharedSentinelState,
}

#[derive(Deserialize)]
pub struct IntentPayload {
    script_payload: String,
}

#[derive(Deserialize)]
pub struct TextIntentPayload {
    intent_text: String,
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
    temporal_resonance: f64,
    probability_confidence: f32,
    generated_script: Option<String>,
    ritual_seal: Option<crate::ledger::RitualSeal>,
}

/// Spawns the Axum API gateway for the Occult Virtual Machine.
pub async fn start_gateway(sentinel_state: SharedSentinelState) {
    let rate_limiter = Arc::new(AtomicUsize::new(0));

    let app_state = AppState {
        rate_limiter,
        sentinel_state,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/sentinel_logs", get(get_sentinel_logs))
        .route("/cast_intent", post(cast_intent))
        .route("/manifest_from_text", post(manifest_from_text))
        .route("/simulate_intent", post(simulate_intent))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3690));
    println!("[ALTAR_GATEWAY] Constructing API bridge. Binding to {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind gateway to loopback network");
    axum::serve(listener, app).await.expect("Axum gateway critical fault");
}

async fn get_sentinel_logs(State(state): State<AppState>) -> Json<Vec<SentinelLog>> {
    let logs = state.sentinel_state.logs.lock().unwrap().clone();
    Json(logs)
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
    State(state): State<AppState>,
    headers: HeaderMap, 
    Json(payload): Json<IntentPayload>,
) -> Result<Json<IntentResponse>, (StatusCode, &'static str)> {
    println!("[ALTAR_GATEWAY] Neural Link Active: Evaluating inbound intent...");

    match headers.get("X-MPL-SIGIL") {
        Some(sigil) if sigil == "369-TESLA-RESONANCE" => {}
        _ => {
            println!("[SECURITY_FAULT] Unauthorized manifestation attempt rejected.");
            return Err((StatusCode::UNAUTHORIZED, "Invalid or missing X-MPL-SIGIL."));
        }
    }

    let requests = state.rate_limiter.fetch_add(1, Ordering::SeqCst);
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
    let mut ovm = OVM::new(432.0, Box::new(AppManifestExecutor::new()), "GATEWAY_CAST");
    if headers.contains_key("X-MPL-SHADOW") {
        ovm.is_shadow_mode = true;
    }
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
        temporal_resonance: ovm.temporal_resonance,
        probability_confidence: ovm.probability_confidence,
        generated_script: None,
        ritual_seal: ovm.last_ritual_seal.clone(),
    }))
}

async fn manifest_from_text(
    State(state): State<AppState>,
    headers: HeaderMap, 
    Json(payload): Json<TextIntentPayload>,
) -> Result<Json<IntentResponse>, (StatusCode, &'static str)> {
    println!("[ALTAR_GATEWAY] NLP Manifestation Engaged: {}", payload.intent_text);

    match headers.get("X-MPL-SIGIL") {
        Some(sigil) if sigil == "369-TESLA-RESONANCE" => {}
        _ => {
            println!("[SECURITY_FAULT] Unauthorized manifestation attempt rejected.");
            return Err((StatusCode::UNAUTHORIZED, "Invalid or missing X-MPL-SIGIL."));
        }
    }

    let requests = state.rate_limiter.fetch_add(1, Ordering::SeqCst);
    if requests > 50 {
        return Err((StatusCode::TOO_MANY_REQUESTS, "Intent flooding detected. Neural pathway saturated."));
    }

    let mut neural_cortex = crate::mlx_engine::NeuralCortex::new();
    let generated_script = neural_cortex.transcode_intent_to_script(&payload.intent_text);

    let lexer = Lexer::new(&generated_script);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut ovm = OVM::new(432.0, Box::new(AppManifestExecutor::new()), &payload.intent_text);
    if headers.contains_key("X-MPL-SHADOW") {
        ovm.is_shadow_mode = true;
    }
    ovm.execute(program);

    let visual_sigil = ovm.last_visual_sigil.clone();
    let generation_version = ovm.evolution_engine.generation;
    let sonic_parameters = Some(crate::stdlib::invoke_sonic_transmutation(ovm.akashic_record.get_temporal_success_rate(), ovm.hz_alignment));

    Ok(Json(IntentResponse {
        status: "Transcoded and Executed".to_string(),
        message: "Neural Cortex successfully mapped human intent to deterministic execution vector.".to_string(),
        visual_sigil,
        sonic_parameters,
        generation_version,
        temporal_resonance: ovm.temporal_resonance,
        probability_confidence: ovm.probability_confidence,
        generated_script: Some(generated_script),
        ritual_seal: ovm.last_ritual_seal.clone(),
    }))
}

async fn simulate_intent(
    State(state): State<AppState>,
    headers: HeaderMap, 
    Json(payload): Json<TextIntentPayload>,
) -> Result<Json<crate::sdk_bridge::SdkSimulateResponse>, (StatusCode, &'static str)> {
    println!("[ALTAR_GATEWAY] Neural Sandbox: Simulating NLP Manifestation...");

    match headers.get("X-MPL-SIGIL") {
        Some(sigil) if sigil == "369-TESLA-RESONANCE" => {}
        _ => {
            println!("[SECURITY_FAULT] Unauthorized simulation attempt rejected.");
            return Err((StatusCode::UNAUTHORIZED, "Invalid or missing X-MPL-SIGIL."));
        }
    }

    let requests = state.rate_limiter.fetch_add(1, Ordering::SeqCst);
    if requests > 50 {
        return Err((StatusCode::TOO_MANY_REQUESTS, "Intent flooding detected. Neural pathway saturated."));
    }

    let mut neural_cortex = crate::mlx_engine::NeuralCortex::new();
    let generated_script = neural_cortex.transcode_intent_to_script(&payload.intent_text);

    let lexer = Lexer::new(&generated_script);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut ovm = OVM::new(432.0, Box::new(AppManifestExecutor::new()), &payload.intent_text);
    // Force shadow mode on for simulate route
    ovm.is_shadow_mode = true;
    ovm.execute(program);

    let resonance_score = ovm.akashic_record.get_temporal_success_rate();

    Ok(Json(crate::sdk_bridge::SdkSimulateResponse::map_from_ovm(&ovm, resonance_score)))
}
