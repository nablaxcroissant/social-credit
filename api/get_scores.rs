use social_credit::Person;
use futures::stream::TryStreamExt;

use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use mongodb::{ 
	bson::doc,
	Client,
	Collection,
    options::FindOptions,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(init).await
}

pub async fn init(_req: Request) -> Result<Response<Body>, Error> {
    let uri = "mongodb+srv://lauercarson:X5mh2OCR9ZCKLc9w@cluster0.vp7unzw.mongodb.net/?retryWrites=true&w=majority";
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;
    // Get a handle on the movies collection
    let database = client.database("registry");
    let people: Collection<Person> = database.collection("people");
    
    let opts: FindOptions = FindOptions::builder()
        .sort(doc! { "score": 1 })
        .build();

    let filter = doc! {};
    let mut cursor = people.find(filter, opts).await?;

    let mut people = Vec::new();

    while let Some(person) = cursor.try_next().await? {
        people.push(person);
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "people": people
            })
            .to_string()
            .into(),
        )?)
}