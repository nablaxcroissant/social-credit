use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
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
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }, None).await?;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "Yessir"
            })
            .to_string()
            .into(),
        )?)
}