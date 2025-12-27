use axum::{Router, routing::get};
use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Rust server is running"
}

async fn mongo_test() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "your_mongodb_atlas_connection_string";
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;
    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie = my_coll
        .find_one(doc! { "title": "The Perils of Pauline" })
        .await?;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://{}", addr);
    println!("Testing MongoDB connection...");
    if let Err(e) = mongo_test().await {
        eprintln!("Error connecting to MongoDB: {}", e);
    }

    axum::serve(listener, app).await.unwrap();
}
