//! Yahoo Finance as an Arvo plugin: the first source to live outside the app
//! (ADR-0022).
//!
//! Everything Yahoo-specific stays in `arvo-yfinance`, inside `arvo-desktop`;
//! this is the process boundary and nothing else. Both venues are served,
//! because they are two datasets rather than one source configured two ways
//! (ADR-0013), and the host files them under two venues exactly as it did
//! when they were compiled in.
//!
//! The address comes from `ARVO_PLUGIN_ADDR`. Arvo's supervisor (ADR-0023)
//! will set it; until then a person does, and the default is a fixed port.

use std::net::SocketAddr;

use arvo_plugin_host::source::{serve, Served, SERVICE};
use arvo_yfinance::Yahoo;

const DEFAULT_ADDR: &str = "127.0.0.1:50052";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = std::env::var("ARVO_PLUGIN_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_owned()).parse()?;
    let plugin = Served::new(
        "yahoo",
        "Yahoo Finance",
        env!("CARGO_PKG_VERSION"),
        vec![Box::new(Yahoo::new()), Box::new(Yahoo::total_return())],
    );
    println!("arvo-plugin-yahoo serving {SERVICE} at {addr}");
    serve(addr, plugin).await?;
    Ok(())
}
