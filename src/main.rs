use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{App, HttpServer};
mod handlers;
mod state;
mod wordle;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 単語リストをロード
    let solutions = state::load_words("src/data/solutions.txt")?;
    let guesses = state::load_words("src/data/guesses.txt")?;
    let state = state::AppState { solutions, guesses };

    HttpServer::new(move || {
        // CORS 設定
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:5173") // フロントのオリジン
            .allowed_origin("http://localhost:5173") // フロントのオリジン
            .allowed_methods(vec!["POST"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors) // ここで wrap
            .app_data(actix_web::web::Data::new(state.clone()))
            .configure(handlers::init)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
