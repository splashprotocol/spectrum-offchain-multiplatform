use crate::analytics::{Analytics, LaunchType};
use crate::re_captcha::{ReCaptcha, ReCaptchaSecret, ReCaptchaToken};
use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use bloom_offchain_cardano::orders::adhoc::beacon_from_oref;
use clap::Parser;
use cml_crypto::{Bip32PrivateKey, PrivateKey, RawBytesEncoding};
use derive_more::From;
use log::{error, info};
use spectrum_cardano_lib::{AssetClass, OutputRef};
use std::time::{SystemTime, UNIX_EPOCH};

mod analytics;
pub mod re_captcha;

#[derive(serde::Serialize, From)]
struct SignatureHex(String);

#[derive(serde::Deserialize, Clone, Debug)]
struct AuthRequest {
    input_oref: OutputRef,
    order_index: u64,
    input_amount: u64,
    input_asset: AssetClass,
    output_asset: AssetClass,
    token: ReCaptchaToken,
}

#[derive(serde::Serialize)]
struct AuthResponse {
    signature: SignatureHex,
}

#[post("/auth")]
async fn auth(
    captcha: Data<ReCaptcha>,
    analytics: Data<Analytics>,
    sk: Data<PrivateKey>,
    limits: Data<Limits>,
    req: web::Json<AuthRequest>,
) -> impl Responder {
    let token_opt = req.output_asset.into_token().or(req.input_asset.into_token());

    // Rules:
    // - if pool launch is `fair`:
    //  1) Captcha verification
    //  2) Token value verification:
    //      - If input is ADA:
    //          * If diff between pool launch and request is lt 3 min - 25 ADA
    //          * If diff between pool launch and request is lt 6 min and gte 3 min - 50 ADA
    //          * If diff between pool launch and request is lt 9 min and gte 6 min - 100 ADA
    //          * If diff between pool launch and request is gt 9 - no limit
    //      - If input is Token always true
    // - if pool launch is `common`:
    //  1) Captcha verification
    let system_time = SystemTime::now();
    let since_the_epoch = system_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    info!(
        "Going to process request {:?} at {}",
        req,
        since_the_epoch.as_millis()
    );
    if !captcha.verify(req.token.clone()).await {
        info!("Captcha verification failed");
        return HttpResponse::Ok().body("Verification failed");
    }
    match token_opt {
        None => HttpResponse::BadRequest().body("ADA/ADA request"),
        Some(token) => match analytics.get_token_pool_info(token).await {
            Ok(pool_info) => {
                let pool_verification_result_is_success: bool = match pool_info.launch_type {
                    LaunchType::Fair => {
                        if req.output_asset.is_native() {
                            true
                        } else {
                            let pool_created_time = pool_info.created_on.as_secs();

                            let diff_between_order_and_pool_creation_in_mins =
                                (since_the_epoch.as_secs() as i64 - pool_info.created_on.as_secs() as i64)
                                    / 60;

                            info!(
                                "Difference between pool creation {} and request time is {} min.",
                                pool_created_time, diff_between_order_and_pool_creation_in_mins
                            );

                            match diff_between_order_and_pool_creation_in_mins {
                                less_than_3_min if less_than_3_min < 3 => {
                                    req.input_amount <= limits.three_min_limit
                                }
                                less_than_6_min if less_than_6_min < 6 => {
                                    req.input_amount <= limits.six_min_limit
                                }
                                less_than_9_min if less_than_9_min < 9 => {
                                    req.input_amount <= limits.nine_min_limit
                                }
                                more_than_9 if more_than_9 >= 9 => true,
                                _ => false,
                            }
                        }
                    }
                    LaunchType::Common => true,
                };
                let response = if pool_verification_result_is_success {
                    let beacon = beacon_from_oref(
                        req.input_oref,
                        req.order_index,
                        req.input_amount,
                        req.input_asset,
                        req.output_asset,
                    );
                    let proof = sk.sign(beacon.to_raw_bytes());
                    let response = AuthResponse {
                        signature: proof.to_raw_hex().into(),
                    };
                    let body = serde_json::to_string(&response).unwrap();
                    HttpResponse::Ok().content_type(ContentType::json()).body(body)
                } else {
                    error!(
                        "pool_verification_result_is_success {} for request {:?}",
                        pool_verification_result_is_success, req
                    );
                    HttpResponse::Ok().body("Verification failed")
                };
                Ok(response)
            }
            Err(err) => {
                error!("Error occured during analytics request {}", err);
                Err(HttpResponse::InternalServerError().body("Internal server error"))
            }
        }
        .unwrap_or(HttpResponse::InternalServerError().body("Internal server error")),
    }
}

#[derive(serde::Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
struct Limits {
    three_min_limit: u64,
    six_min_limit: u64,
    nine_min_limit: u64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
    re_captcha_secret: ReCaptchaSecret,
    secret_bech32: String,
    analytics_snek_url: String,
    limits: Limits,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = AppArgs::parse();

    log4rs::init_file(args.log4rs_path, Default::default()).unwrap();

    let raw_config = std::fs::File::open(args.config_path).expect("Cannot load configuration file");
    let config: AppConfig = serde_json::from_reader(raw_config).expect("Invalid configuration file");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        let re_captcha = Data::new(ReCaptcha::new(config.re_captcha_secret.clone()));
        let analytics = Data::new(Analytics::new(config.analytics_snek_url.clone()));
        let sk = Data::new(
            Bip32PrivateKey::from_bech32(config.secret_bech32.as_str())
                .expect("Invalid secret bech32")
                .to_raw_key(),
        );
        App::new()
            .wrap(cors)
            .app_data(re_captcha)
            .app_data(analytics)
            .app_data(sk)
            .app_data(Data::new(config.limits))
            .service(auth)
    })
    .bind((args.host, args.port))?
    .workers(8)
    .run()
    .await
}

#[derive(Parser)]
#[command(name = "snek-auth-server")]
#[command(author = "Spectrum Labs")]
#[command(version = "1.0.0")]
#[command(about = "Snek Auth Server", long_about = None)]
struct AppArgs {
    /// Path to the JSON configuration file.
    #[arg(long, short)]
    config_path: String,
    #[arg(long, short)]
    log4rs_path: String,
    #[arg(long)]
    host: String,
    #[arg(long)]
    port: u16,
}
