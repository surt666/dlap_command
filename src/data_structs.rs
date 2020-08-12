use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Types {
    Dataset,
    Profile,
    User,
    Account,
    Subset,
    Edge,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Criticality {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Confidentiality {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dataset {
    pub name: String,
    pub pk: String,
    pub sk: String,
    pub gsi1_pk: String,
    pub gsi1_sk: String,
    pub r#type: Types,
    pub created: u64,
    pub cost_center: Option<String>,
    pub owner: String,
    pub criticality: Criticality,
    pub confidentiality: Confidentiality,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Subset {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub pk: String,
    pub sk: String,
    pub gsi1_pk: String,
    pub gsi1_sk: String,
    pub r#type: Types,
    pub created: u64,

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {

}
