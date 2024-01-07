// #[warn(unused_imports)]
// use actix_web::{web, App, HttpServer, HttpResponse};
// use std::path::PathBuf;
// #[warn(unused_imports)]
// use anyhow::Result;
//
//
// pub async fn index() -> Result<HttpResponse, actix_web::Error>{
//     let mut path = PathBuf::from(".");
//     path.push("index.html");
//     let content = std::fs::read_to_string(path)?;
//
//     Ok(HttpResponse::Ok()
//         .content_type("text/html")
//         .body(content)
//     )
// }
//
// pub async fn server() -> Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//     })
//         //Change your Port no Here // Use sudo if using ports below 1024 especially on linux.
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await?;
//     Ok(())
// }

#[warn(unused_imports)]
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, error::Error as ActixError};
use std::path::PathBuf;
use anyhow::Result;
use nix::unistd::{fork, ForkResult};


pub async fn index(_req: HttpRequest) -> Result<HttpResponse, ActixError> {
    let path = PathBuf::from("payload/index.html");
    let content = std::fs::read_to_string(&path)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(content)

    )
}

pub async fn js_file(_req: HttpRequest) -> Result<HttpResponse, ActixError> {
    let path = PathBuf::from("payload/new_locate.js");
    let content = std::fs::read_to_string(&path)?;

    Ok(HttpResponse::Ok()
        .content_type("application/javascript")
        .body(content)
    )
}

async fn error_handle(err: ActixError, _req: HttpRequest) -> ActixError {
    actix_web::error::InternalError::from_response(err, HttpResponse::InternalServerError().finish()).into()
}

// child server for subprocessing
pub async fn child_server() -> Result<(), anyhow::Error> {
    match unsafe {fork ()}{
        Ok(ForkResult::Parent {child, ..}) => {
            println!("Server running in background on port http://localhost:8080, on PID : {:?}", child);
            return Ok(());
        }
        Ok(ForkResult::Child) => {
            HttpServer::new(|| {
                App::new()
                    .route("/", web::get().to(index))
                    .route("/new_locate.js", web::get().to(js_file))
                    .app_data(web::Data::new(|| error_handle))
            })
                // Change you IP or Port no if needed . For Linux using below 1024 port requires special permission -> use sudo to run the program
                .bind("127.0.0.1:8080")?
                .run()
                .await?;
            Ok(())
        }
        Err(_) => {
            return Err(anyhow::Error::msg("Failed to background process"));
        }
    }
}

// parent_server for localhost only
pub async fn parent_server() -> Result<(), anyhow::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/new_locate.js", web::get().to(js_file))
            .app_data(web::Data::new(|| error_handle))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}

