mod data_structs;

use crate::data_structs::{Dataset, User, Profile, Account, Subset, Types, Edge};
use ddb_util::{batch_write_items, put_item, query, set_kv, DdbMap};
use itertools::Itertools;
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDbClient, WriteRequest, AttributeValue};
use serde::{Deserialize, Serialize};
use serde_diff::{Apply, Diff, SerdeDiff};
use serde_json::json;
use simple_logger;
use std::collections::HashMap;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use aws_lambda_events::event::apigw::ApiGatewayV2httpRequest;
use log::{self, error};
use simple_error::bail;
use lazy_static::*;


lazy_static! {
    static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::default());
}

const RELATIONS_TABLE: &str = "dsaccess";

type Name = String;
type PK = String;
type UserProfile = String;


#[derive(Deserialize, Serialize, Debug)]
enum Actions {
    CreateDataset(Dataset),
    UpdateDataset(Dataset),
    DeleteDataset(Name),
    CreateUser(User),
    UpdateUser(User),
    DeleteUser(Name),
    CreateSubset(Subset),
    DeleteSubset(Name),
    CreateAccount(Account),
    DeleteAccount(Name),
    CreateProfile(Profile),
    CreateEdge(PK, PK),
    CreateProfileEdge(PK, PK, UserProfile)
}

#[derive(Deserialize, Serialize, Debug)]
struct ActionEvent {
    action: Actions,
}

#[derive(Serialize, Debug)]
struct Headers {
    content_type: String,
}

#[derive(Serialize, Debug)]
struct EntityOutput {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: i32,
    headers: Headers,
    body: String,
}

fn generate_lambda_output(body: HashMap<String, String>, return_code: i32) -> EntityOutput {
    EntityOutput {
        is_base64_encoded: false,
        status_code: return_code,
        headers: Headers {
            content_type: "application/json".to_string(),
        },
        body: serde_json::to_string(&body).expect("Could not handle input"),
    }
}

fn now_as_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn clean_item(item: DdbMap) -> DdbMap {
    let mut i = item.clone();
    for (key, value) in item.into_iter() {
	if let Some(_x) = value.null {
	    i.remove(&key);
	}
	if let Some(x) = value.m {
	    i.remove(&key);
	    let v = x.get(&"___enum_tag".to_string()).unwrap();
	    i.insert(key, v.clone());
	}
    }
    i
}

fn generate_edge_item(pk1: String, pk2: String, rel: String, profile: Option<String>) -> DdbMap {
    let mut ddb_map: DdbMap = DdbMap::new();
    set_kv(&mut ddb_map, "pk".to_string(), pk1);
    set_kv(&mut ddb_map, "sk".to_string(), format!("{}#{}", rel, pk2));
    set_kv(&mut ddb_map, "created".to_string(), now_as_secs().to_string());
    set_kv(&mut ddb_map, "type".to_string(), Types::Edge.to_string());
    if let Some(p) = profile {
	set_kv(&mut ddb_map, "profile".to_string(), format!("P#{}", p));
    }
    ddb_map
}

fn generate_edge_items(edge: Edge) -> Result<Vec<DdbMap>, HandlerError> {
    match (edge.pk1_type, edge.pk2_type) {
	(Types::Dataset, Types::User) => {
	    let ddb1 = generate_edge_item(edge.pk1.clone(), edge.pk2.clone(), "has_member".to_string(), edge.profile.clone());
	    let ddb2 = generate_edge_item(edge.pk2, edge.pk1, "is_member_of".to_string(), edge.profile);	 
	    Ok(vec![ddb1, ddb2])
	},
	(Types::Dataset, Types::Account) => {
	    let ddb1 = generate_edge_item(edge.pk1.clone(), edge.pk2.clone(), "has_member".to_string(), edge.profile.clone());
	    let ddb2 = generate_edge_item(edge.pk2, edge.pk1, "is_member_of".to_string(), edge.profile);	 
	    Ok(vec![ddb1, ddb2])
	},
	(Types::Dataset, Types::Subset) => {
	    let ddb1 = generate_edge_item(edge.pk1.clone(), edge.pk2.clone(), "has_subset".to_string(), edge.profile);	   	 
	    Ok(vec![ddb1])
	},
	(Types::Dataset, Types::Steward) => {
	    let ddb1 = generate_edge_item(edge.pk1.clone(), edge.pk2.clone(), "has_steward".to_string(), edge.profile.clone());
	    let ddb2 = generate_edge_item(edge.pk2, edge.pk1, "is_steward_of".to_string(), edge.profile);	 
	    Ok(vec![ddb1, ddb2])
	}
	_ => {
	    error!("PK types not comatible with edge");
            bail!("PK types not comatible with edge");
	}, 
    }
}

#[tokio::main]
async fn handler(e: ApiGatewayV2httpRequest, _c: Context) -> Result<EntityOutput, HandlerError> {
    println!("E {:#?}", e);
    let ae: ActionEvent = serde_json::from_str(&e.body.unwrap())?;
    match ae.action {
        Actions::CreateDataset(mut ds) => {
            ds.created = Some(now_as_secs());
            ds.pk = Some(format!("D#{}", ds.name));
            ds.sk = ds.pk.clone();
	    ds.gsi1_pk = Some("dataset".to_string());
	    ds.gsi1_sk = ds.pk.clone();
            ds.r#type = Some(Types::Dataset);
            let item: DdbMap = serde_dynamodb::to_hashmap(&ds).unwrap();
	    let cleaned_item = clean_item(item);
            println!("DDB Item {:#?}", cleaned_item);
            let res = ddb_util::put_item(&DYNAMODB, RELATIONS_TABLE, cleaned_item).await;
            println!("{:#?}", res);
       	    let mut res = HashMap::new();  // MOVE TO generate_lambda_output
	    match ds.pk {
		Some(x) => {
		    res.insert("pk".to_string(), x);
		    Ok(generate_lambda_output(res, 200))
		},
		None => {
		    res.insert("status".to_string(), "pk is unset".to_string());
		    Ok(generate_lambda_output(res, 500))
		}
	    }
            
        },
	Actions::CreateUser(mut user) => {	    
            user.created = Some(now_as_secs());
            user.pk = Some(format!("U#{}", user.name));
            user.sk = user.pk.clone();
	    user.gsi1_pk = Some("user".to_string());
	    user.gsi1_sk = user.pk.clone();
            user.r#type = Some(Types::User);
            let item: DdbMap = serde_dynamodb::to_hashmap(&user).unwrap();
	    let cleaned_item = clean_item(item);
            println!("DDB Item {:#?}", cleaned_item);
            let res = ddb_util::put_item(&DYNAMODB, RELATIONS_TABLE, cleaned_item).await;
            println!("{:#?}", res);
       	    let mut res = HashMap::new();  // MOVE TO generate_lambda_output
	    if let Some(x) = user.pk {
		res.insert("pk".to_string(), x);
	    };
            Ok(generate_lambda_output(res, 200))
        },
	Actions::CreateEdge(pk1, pk2) => {
            let edge = Edge::new(pk1, pk2);	  
	    let items = generate_edge_items(edge)?;	
            let res = ddb_util::batch_write_items(&DYNAMODB, RELATIONS_TABLE, Some(items), None).await;
            println!("{:#?}", res);
            let mut res2 = HashMap::new();  // MOVE TO generate_lambda_output
	    res2.insert("status".to_string(), "Edge created".to_string());
            Ok(generate_lambda_output(res2, 200))
        },
	Actions::CreateProfileEdge(pk1, pk2, profile) => {
            let mut edge = Edge::new(pk1, pk2);	    
	    edge.profile = Some(profile);	    
	    let items = generate_edge_items(edge)?;	
            let res = ddb_util::batch_write_items(&DYNAMODB, RELATIONS_TABLE, Some(items), None).await;
            println!("{:#?}", res);
            let mut res2 = HashMap::new();  // MOVE TO generate_lambda_output
	    res2.insert("status".to_string(), "Edge created".to_string());
            Ok(generate_lambda_output(res2, 200))
        },
	_ => {
	    let mut res = HashMap::new();
	    res.insert("status".to_string(), "Not implemented".to_string());
	    Ok(generate_lambda_output(res, 400))
	}
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Warn)?;
    lambda!(handler);
    Ok(())
}
