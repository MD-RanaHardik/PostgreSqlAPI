use actix_web::{web, App, HttpServer};
mod views;

use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::{self, NoTls};
use views::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut cfg = Config::new();
    cfg.dbname = Some("Employee".to_string());
    cfg.password = Some("root".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
