use std::ops::DerefMut;

use rocket::{
    fairing::{self, AdHoc},
    serde::{json::Json, Serialize},
    Build, Rocket,
};
use rocket_db_pools::{
    deadpool_postgres::{self, tokio_postgres::Row},
    Connection, Database,
};

#[macro_use]
extern crate rocket;

pub trait Entity {
    fn from(row: Row) -> Self;
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Table {
    id: i32,
    name: String,
}

impl Entity for Table {
    fn from(row: Row) -> Self {
        return Self {
            id: row.get("id"),
            name: row.get("name"),
        };
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TableOut {
    tables: Vec<Table>,
}

#[get("/")]
async fn index(db: Connection<MyDatabase>) -> Json<Vec<Table>> {
    let stuff_entries = db
        .query(
            &db.prepare("SELECT * FROM stuff WHERE name=$1;")
                .await
                .unwrap(),
            &[&"do_stuff".to_string()],
        )
        .await
        .unwrap();

    let mut results: Vec<Table> = vec![];
    for row in stuff_entries {
        results.push(Entity::from(row));
    }

    return Json(results);
}

#[derive(Database)]
#[database("public")]
struct MyDatabase(deadpool_postgres::Pool);

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("sql");
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = MyDatabase::fetch(&rocket) {
        let mut conn = db.0.get().await.unwrap();
        let client = conn.deref_mut().deref_mut();
        let report = embedded::migrations::runner().run_async(client).await;
        println!("{:#?}", report);
        Ok(rocket)
    } else {
        Err(rocket)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyDatabase::init())
        .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
        .mount("/", routes![index])
}
