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
    Critical,
    NonCritical,                         
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Confidentiality {
    StrictlyConfidential,
    Confidential,
    Internal,
    Public,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dataset {
    pub name: String,
    pub pk: Option<String>,
    pub sk: Option<String>,
    pub gsi1_pk: Option<String>,
    pub gsi1_sk: Option<String>,
    pub r#type: Option<Types>,
    pub created: Option<u64>,
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
    pub pk: Option<String>,
    pub sk: Option<String>,
    pub gsi1_pk: Option<String>,
    pub gsi1_sk: Option<String>,
    pub r#type: Option<Types>,
    pub created: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {

}
