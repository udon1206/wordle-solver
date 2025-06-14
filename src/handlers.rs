use crate::state::AppState;
use crate::wordle::propose_optimal_string;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// リクエスト JSON
#[derive(Deserialize)]
pub struct NextGuessRequest {
    pub answer_strings: Vec<String>,
    pub answer_statuses: Vec<String>,
}

/// レスポンス JSON
#[derive(Serialize)]
pub struct NextGuessResponse {
    pub next_guess: String,
}

/// ルート設定
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/next_guess", web::post().to(next_guess_handler));
    cfg.service(root_handler);
}

/// 次の推測を返すハンドラ
async fn next_guess_handler(
    data: web::Data<AppState>,
    req: web::Json<NextGuessRequest>,
) -> impl Responder {
    let next = propose_optimal_string(
        &req.answer_strings,
        &req.answer_statuses,
        &data.solutions,
        &data.guesses,
    );
    HttpResponse::Ok().json(NextGuessResponse { next_guess: next })
}

#[get("/")]
pub async fn root_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}
