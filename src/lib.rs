use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person{
    pub first_name: String,
    pub last_name: String,
    pub score: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportReq{
    pub first_name: String,
    pub last_name: String,
    pub update: i32,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub violator: Person,
    pub update: i32,
    pub reason: String,
}