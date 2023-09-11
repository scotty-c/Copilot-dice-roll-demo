use warp::{Filter, Reply};
use rand::Rng;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct RollResult {
    dice: u32,
    rolls: Vec<u32>,
    total: u32,
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "0.0.0.0:80".parse().unwrap(); // Bind to all available network interfaces

    // Define a route for rolling dice
    let roll_dice = warp::path!("roll" / u32)
        .map(|num_dice| {
            let mut rng = rand::thread_rng();
            let rolls: Vec<u32> = (0..num_dice).map(|_| rng.gen_range(1..=6)).collect();
            let total: u32 = rolls.iter().sum();
            let roll_result = RollResult {
                dice: num_dice,
                rolls,
                total,
            };
            warp::reply::json(&roll_result)
        });

    let index = warp::path::end()
        .map(|| {
            warp::reply::html(
                r#"
                <html>
                    <head>
                        <title>Roll Dice</title>
                    </head>
                    <body>
                        <h1>Roll Dice</h1>
                        <p>Roll some dice by visiting <code>/roll/&lt;number&gt;</code></p>
                    </body>
                </html>
                "#,
            )
        });    

    // Combine the filters into a warp service
    let routes = roll_dice.or(index);

    // Start the warp server
    warp::serve(routes)
        .run(addr)
        .await;
}