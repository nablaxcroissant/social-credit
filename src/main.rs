use social_credit::{Person, Report};
use futures::stream::TryStreamExt;

use mongodb::{ 
	bson::doc,
	Client,
	Collection,
    options::FindOptions,
};
use vercel_runtime::{Request, Body};

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
    // people.insert_one(new_user.clone(), None).await?;
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

    let reports: Collection<Report> = database.collection("reports");

    let report = Report {
        first_name: "Carson".to_string(),
        last_name: "Lauer".to_string(),
        update: -20,
        reason: "Crimes".to_owned(),
    };

    let req = Request::new(Body::Text(serde_json::to_string(&report).unwrap()));

    if let Body::Text(req_str) = req.body() {
        let new_report: Report = serde_json::from_str(&req_str).unwrap();

        // reports.insert_one(new_report, None).await?;
    }
    

    // Print the document
    println!("Found a movie:\n{:#?}", people);
    Ok(())
}