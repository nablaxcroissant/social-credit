use social_credit::Person;
use futures::stream::TryStreamExt;

use mongodb::{ 
	bson::doc,
	Client,
	Collection,
    options::FindOptions,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb+srv://lauercarson:X5mh2OCR9ZCKLc9w@cluster0.vp7unzw.mongodb.net/?retryWrites=true&w=majority";
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;
    // Get a handle on the movies collection
    let database = client.database("registry");
    let people: Collection<Person> = database.collection("people");
    let new_user = Person {
        first_name: "Carson".to_string(),
        last_name: "Lauer".to_string(),
        score: 1000
    };
    people.insert_one(new_user, None).await?;
    // Find a movie based on the title value
    // let me = people.find_one(doc! { "last_name": "Lauer" }, None).await?;

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

    // Print the document
    println!("Found a movie:\n{:#?}", people);
    Ok(())
}