// from actix-web
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
// log
use log::info;

use crate::ip::Ip;

mod ip;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    info!("Hello, world");

    let addr: String;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        addr = format!("0.0.0.0:{}", args[1]);
    } else {
        addr = "0.0.0.0:13852".to_string();
    }

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handle_ip))
            .route("/{name}", web::get().to(handle_ip))
    })
    .bind(addr)?
    .run()
    .await
}

async fn handle_ip(req: HttpRequest) -> impl Responder {
    let ip_string = req.peer_addr().unwrap().ip().to_string();
    let url = format!("https://api.ip.sb/geoip/{}", ip_string);
    let ip: Ip = reqwest::get(url.as_str())
        .await
        .unwrap()
        .json::<Ip>()
        .await
        .unwrap();
    info!("a new client from {}", &ip.ip);
    let name = req.match_info().get("name").unwrap_or("World");
    format!(
        "Hello {}!\n\
    Your are from {}\n\
    Your IP is {}\n\
    Your ISP is {}",
        &name, &ip.country, &ip.ip, &ip.isp
    )
}

fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}
