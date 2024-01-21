use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person{
    pub first_name: String,
    pub last_name: String,
    pub score: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report{
    pub violator: Person,
    pub reporter: Person,
    pub points_reduced: u32,
    pub reason: String,
}