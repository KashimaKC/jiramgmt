
use mongodb::{
    bson::{Document, doc},
    sync::{Client, Collection}
  };

fn start_connection() -> Result<mongodb::sync::Collection<mongodb::bson::Document>, String> {
   let uri = "mongodb://localhost:27017";
   let client = Client::with_uri_str(uri).map_err(|e| e.to_string())?;
   let db = client.database("jmdb");
   let collection = db.collection("jiramgmt");

   Ok(collection)
}

#[tauri::command]
pub fn insert_db(date: String, task: String) {
    let connection = start_connection();
    let db = connection.unwrap();

    let _insert_result = db.insert_one(doc! 
        { 
            "date": date,
            "task": task
        }, None
    );
}

#[tauri::command]
pub fn retrieve_logs() -> Result<String, String> {
    let connection = start_connection();
    let db = connection.unwrap();
    let cursor = db.find(doc! {}, None).expect("failed to run query.");
    let documents = cursor.collect::<Result<Vec<_>, _>>();
    let json = serde_json::to_string(&documents.unwrap());

    Ok(json.unwrap())
}