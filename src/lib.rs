// use actix_web::dev::Server;
// use actix_web::{web, App, HttpResponse, HttpServer};
// use std::net::TcpListener;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// // Let's start simple: we always return a 200 OK
// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }

/// A trait encapsulating the operations required of a logger.
// pub trait Log: Sync + Send {
//     /// Determines if a log message with the specified metadata would be
//     /// logged.
//     ///
//     /// This is used by the `log_enabled!` macro to allow callers to avoid
//     /// expensive computation of log message arguments if the message would be
//     /// discarded anyway.
//     fn enabled(&self, metadata: &Metadata) -> bool;

//     /// Logs the `Record`.
//     ///
//     /// Note that `enabled` is *not* necessarily called before this method.
//     /// Implementations of `log` should perform all necessary filtering
//     /// internally.
//     fn log(&self, record: &Record);

//     /// Flushes any buffered records.
//     fn flush(&self);
// }
pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;
