pub mod types;

use {
    anyhow::{anyhow, Context},
    axum::{
        http::StatusCode,
        response::IntoResponse,
        routing::post,
        Json, Router,
    },
    clap::{Arg, Command},
    ix_decoder::DecodeMatcher,
    solana_transaction_status::UiInstruction,
    types::{DecodeInstruction, Error},
};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let matches = Command::new("decoder-api")
        .about("cli for the solana instruction decoder api")
        .subcommands(vec![Command::new("start")
            .about("start the decoder api")
            .arg(
                Arg::new("listen-url")
                    .long("listen-url")
                    .help("url to expose api on")
                    .default_value("127.0.0.1:3000"),
            )])
        .get_matches();
    match matches.subcommand() {
        Some(("start", s)) => {
            let listen_url = s.get_one::<String>("listen-url").unwrap();
            serve_decoder_api(listen_url).await
        }
        _ => Err(anyhow!("invalid subcommand")),
    }
}

pub async fn serve_decoder_api(listen_url: &str) -> anyhow::Result<()> {
    let app = Router::new().route("/decode", post(decode_instruction));
    let listener = tokio::net::TcpListener::bind(listen_url).await?;
    axum::serve(listener, app)
        .await
        .with_context(|| "api failed")
}

async fn decode_instruction(Json(payload): Json<DecodeInstruction>) -> impl IntoResponse {
    const DECODE_MATCHER: DecodeMatcher = DecodeMatcher {};
    let instruction: UiInstruction = Into::into(payload);
    match DECODE_MATCHER.try_new_decoder(instruction) {
        Ok(decoder) => match decoder.decode() {
            Ok(decoded) => return (StatusCode::OK, Json(decoded)).into_response(),
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Error {
                        msg: format!("failed to decode instruction {err:#?}"),
                    }),
                )
                    .into_response()
            }
        },
        Err(err) => {
            // log
            return (
                StatusCode::BAD_REQUEST,
                Json(Error {
                    msg: err.to_string(),
                }),
            )
                .into_response();
        }
    }
}
