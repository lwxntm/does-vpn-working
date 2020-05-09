// from actix-web
use actix_web::{App, HttpRequest, HttpServer, Responder, web};
// log
use log::info;
use crate::ip::Ip;

mod ip;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    info!("Hello, world");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handle_ip))
            .route("/{name}", web::get().to(handle_ip))
    })
        .bind("0.0.0.0:13852")?
        .run()
        .await
}

async fn handle_ip(req: HttpRequest) -> impl Responder {
    let ip_string = req.peer_addr().unwrap().ip().to_string();
    // let url = format!("https://api.ip.sb/geoip/{}", ip_string);
    let url ="https://api.ip.sb/geoip/185.209.84.53";
    let ip:ip::Ip=reqwest::get(url).await.unwrap().json::<Ip>().await.unwrap();
    println!("{}",ip.country);
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!Your are from {}", &name,&ip.country)
}

#[test]
fn t() {
    let body: reqwest::blocking::Response = reqwest::blocking::get("https://www.rust-lang.org").unwrap();

    println!("body = {:?}", body);
}

fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
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
