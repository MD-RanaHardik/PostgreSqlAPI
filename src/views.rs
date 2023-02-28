use actix_web::{
    get,
    web::{self, Form},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDataInsert {
    Username: String,
    Password: String,
    Employee_name: String,
    Employee_salary: i32,
    Employee_designation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDataUpdate {
    Employee_name: String,
    Employee_salary: i32,
    Employee_designation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeData {
    id: i32,
    Username: String,
    Password: String,
    Employee_name: String,
    Employee_salary: i32,
    Employee_designation: String,
}

#[get("/index")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("<h1>Hello</h1>")
}

#[get("/insert")]
pub async fn add_new_employee(
    data: Form<EmployeeDataInsert>,
    cradential: BasicAuth,
    pool: web::Data<Pool>,
) -> impl Responder {
    if cradential.user_id() == "Hardik" && cradential.password().unwrap() == "Hardik@123" {
        let client = pool.get().await.unwrap();

        let res = client.execute("insert into Employees (Employee_name,Employee_salary,Employee_designation,Username,Password) values($1,$2,$3,$4,$5)", &[&data.Employee_name,&data.Employee_salary,&data.Employee_designation,&data.Username,&data.Password]).await;

        match res {
            Ok(v) => {
                if v == 1 {
                    HttpResponse::Ok().body("New employee successfully added.")
                } else {
                    HttpResponse::InternalServerError().body("Faild to insert data")
                }
            }
            Err(e) => HttpResponse::InternalServerError().body("Faild to insert data"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}

#[get("/users")]
pub async fn get_all_employeedata(cradential: BasicAuth, pool: web::Data<Pool>) -> impl Responder {
    if cradential.user_id() == "Hardik" && cradential.password().unwrap() == "Hardik@123" {
        let client = pool.get().await.unwrap();
        let mut allemployees: Vec<EmployeeData> = vec![];

        let res = client.query("select * from Employees", &[]).await;

        match res {
            Ok(v) => {
                for i in v {
                    let emp = EmployeeData {
                        id: i.get(0),
                        Employee_name: i.get(1),
                        Employee_salary: i.get(2),
                        Employee_designation: i.get(3),
                        Username: i.get(4),
                        Password: i.get(5),
                    };

                    allemployees.push(emp);
                }

                let sdata = serde_json::to_string_pretty(&allemployees).unwrap();

                HttpResponse::Ok().body(sdata)
            }
            Err(e) => HttpResponse::InternalServerError().body("Faild to get data"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}

#[get("/users/{username}")]
pub async fn get_perticuler_user(
    username: web::Path<String>,
    cradential: BasicAuth,
    pool: web::Data<Pool>,
) -> impl Responder {
    if cradential.user_id() == "Hardik" && cradential.password().unwrap() == "Hardik@123" {
        let client = pool.get().await.unwrap();

        let mut allemployees: Vec<EmployeeData> = vec![];
        let res = client
            .query(
                "select *from Employees where Username = $1",
                &[&username.to_string()],
            )
            .await;

        match res {
            Ok(v) => {
                for i in v {
                    let emp = EmployeeData {
                        id: i.get(0),
                        Employee_name: i.get(1),
                        Employee_salary: i.get(2),
                        Employee_designation: i.get(3),
                        Username: i.get(4),
                        Password: i.get(5),
                    };

                    allemployees.push(emp);
                }

                let sdata = serde_json::to_string_pretty(&allemployees).unwrap();

                HttpResponse::Ok().body(sdata)
            }
            Err(e) => HttpResponse::InternalServerError().body("Faild to get data"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}

#[get("/update/{username}")]
pub async fn update_employee_data(
    username: web::Path<String>,
    cradential: BasicAuth,
    data: web::Form<EmployeeDataUpdate>,
    pool: web::Data<Pool>,
) -> impl Responder {
    if cradential.user_id() == "Hardik" && cradential.password().unwrap() == "Hardik@123" {
        let client = pool.get().await.unwrap();

        let res = client.execute("update Employees set Employee_name =$1 , Employee_salary =$2 ,Employee_designation = $3 where Username =$4", &[&data.Employee_name,&data.Employee_salary,&data.Employee_designation,&username.to_string()]).await;

        match res {
            Ok(r) => {
                if r != 0 {
                    HttpResponse::Ok().body("Record updated")
                } else {
                    HttpResponse::Ok().body("No record found")
                }
            }
            Err(e) => HttpResponse::InternalServerError().body("Faild to update data"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}

#[get("/delete/{username}")]
pub async fn delete_employee_data(
    username: web::Path<String>,
    cradential: BasicAuth,
    pool: web::Data<Pool>,
) -> impl Responder {
    if cradential.user_id() == "Hardik" && cradential.password().unwrap() == "Hardik@123" {
        let client = pool.get().await.unwrap();

        let res = client
            .execute(
                "delete from Employees where Username = $1",
                &[&username.to_string()],
            )
            .await;

        match res {
            Ok(r) => {
                if r != 0 {
                    HttpResponse::Ok().body("Employee deleted")
                } else {
                    HttpResponse::Ok().body("No record found")
                }
            }
            Err(e) => HttpResponse::InternalServerError().body("Faild to delete data"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}
