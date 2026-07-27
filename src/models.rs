use serde::Serialize;



#[derive(Serialize)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
}