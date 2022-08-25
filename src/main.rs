use anyhow::Result;
use reqwest::Client;

const SOLVERS: &[&str] = &[
    "otex",
    "quasimodo",
    "seasolver",
    "plm",
    "stakecapital",
    "atlas",
    "re7",
    "jeffreyliang",
];

async fn get_current_auction(client: &Client) -> Result<String> {
    client
        .get("https://api.cow.fi/mainnet/api/v1/auction")
        .send()
        .await?
        .text()
        .await
        .map_err(anyhow::Error::from)
}

async fn call_driver(
    client: &Client,
    solver: &str,
    endpoint: &str,
    payload: String,
) -> Result<String> {
    client
        .post(format!("http://localhost:8080/api/{solver}/{endpoint}"))
        .body(payload)
        .send()
        .await?
        .text()
        .await
        .map_err(anyhow::Error::from)
}

async fn run_auction(client: Client, solver: &str, auction: String) -> Result<()> {
    let solve_response = call_driver(&client, solver, "solve", auction).await?;
    println!("{solver} solve_response: {solve_response}");
    if solve_response.starts_with("{\"surplus") {
        let execute_response = call_driver(&client, solver, "execute", solve_response).await?;
        println!("{solver} execute_response: {execute_response}");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();

    loop {
        let auction = get_current_auction(&client).await?;
        println!("auction: {auction}");

        let auction_runs = SOLVERS.iter().map(|solver| {
            let auction = auction.clone();
            let client = client.clone();
            run_auction(client, solver, auction)
        });

        futures::future::join_all(auction_runs).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        println!("\n\n\n\n\n");
    }
}
