use social_credit::{Person, ReportReq, Report};

use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use mongodb::{ 
	bson::doc,
	Client,
	Collection,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(report).await
}

pub async fn report(req: Request) -> Result<Response<Body>, Error> {
    let uri = "mongodb+srv://lauercarson:X5mh2OCR9ZCKLc9w@cluster0.vp7unzw.mongodb.net/?retryWrites=true&w=majority";
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;
    // Get a handle on the movies collection
    let database = client.database("registry");
    let people_db: Collection<Person> = database.collection("people");
    let reports_db: Collection<Report> = database.collection("reports");

    if let Body::Text(req_str) = req.body() {
        let report: ReportReq = serde_json::from_str(&req_str).unwrap();

        let violator = people_db.find_one(doc! { "last_name": report.last_name.clone() }, None).await?.unwrap();

        let score = violator.score + report.update;

        let update =
            doc! {
                "$set": doc!{ "score": score }
        };

        let filter = doc!{
            "last_name": violator.last_name.clone(),
        };

        people_db.update_one(
            filter, update, None
        ).await?;

        let report = Report {
            violator: violator.clone(),
            update: score,
            reason: report.reason,
        };

        reports_db.insert_one(report, None).await?;

        Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "violator": violator,
              "score": score,
            })
            .to_string()
            .into(),
        )?)
    } else {
        Ok(Response::builder()
        .status(StatusCode::OK)
        .body(().into())?)
    }
}