
use std::sync::Mutex;

use actix_web::{services, web, App, HttpServer};
mod views;

use tokio_postgres::{Client, Error, NoTls};
use views::index;

use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let (client, connection) =
    //     tokio_postgres::connect("postgresql://postgres:root@localhost/Employee", NoTls)
    //         .await
    //         .expect("Faild to connect with postgres server");

    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });

    HttpServer::new(|| {
        App::new()
            .service(views::index)
            .service(views::add_new_employee)
            .service(views::get_all_employeedata)
            .service(views::get_perticuler_user)
            .service(views::update_employee_data)
            .service(views::delete_employee_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;


    Ok(())
}
