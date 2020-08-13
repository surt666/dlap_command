use serde::{Deserialize, Serialize};
use strum_macros::{Display};

#[derive(Deserialize, Serialize, Debug, Clone, Display)]
pub enum Types {
    Dataset,
    Profile,
    User,
    Account,
    Subset,
    Edge,
    Steward,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Edge {
    pub pk1: String,
    pub pk2: String,
    pub profile: Option<String>,
    pub pk1_type: Types,
    pub pk2_type: Types,
}

fn get_type(pk: &String) -> Types {
    let t: Vec<&str> = pk.split("#").collect();
    match t.first() {
	Some(&"D") => Types::Dataset,
	Some(&"U") => Types::User,
	Some(&"A") => Types::Account,
	Some(&"P") => Types::Profile,
	Some(&"S") => Types::Subset,
	_ => Types::Edge,
    }
}

impl Edge {
    pub fn new(pk1: String, pk2: String) -> Edge {
	Edge {
	    pk1_type: get_type(&pk1),
	    pk2_type: get_type(&pk2),
	    profile: None,
	    pk1,
	    pk2,
	}
    }
}
