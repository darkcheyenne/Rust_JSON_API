// Feature Flags
#![feature(proc_macro_hygiene)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]

// Externe Crates
#[macro_use] extern crate rocket;

// Verwendungen
use mysql as my;
use serde::{Deserialize, Serialize};
use rocket::response::content;

// Statische Variablen
static MYSQL_STRING: &str = "mysql://rusty:12345@127.0.0.1:3306";

// Private Klasse für die Daten in der Tabelle motivcupcakes
#[derive(Serialize, Deserialize)]
struct Motivcupcake {
    id: i32,
    titel: Option<String>,
    preis: f32,
}

#[get("/")]
fn index() -> content::Json<String> {
    // Aufbau der Datenbankverbindung
    let pool = my::Pool::new(MYSQL_STRING).unwrap();

    // Abruf der Daten von der Datenbank
    let selected_stock: Vec<Motivcupcake> =
    pool.prep_exec("SELECT id, titel, preis FROM database.`stock`", ())
    .map(|result| {result.map(|x| x.unwrap()).map(|row| {
            let (id, titel, preis) = my::from_row(row);
            Motivcupcake {
                id: id,
                titel: titel,
                preis: preis,
            }
        }).collect()
    }).unwrap();

    // Umwandeln von Datenbank-Query in JSON
    let serialized = serde_json::to_string(&selected_stock).unwrap();

    // Senden von JSON an Client
    content::Json(serialized)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}