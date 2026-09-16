//! The plugin serves what the compiled-in source served: two venues, two
//! datasets, each on the basis it declares. No network: Describe and Connected
//! are answered from the source's own declarations.

use std::time::Duration;

use arvo_data::source::Source;
use arvo_data::{BarInterval, IntervalUnit};
use arvo_plugin_host::source::v1::source_client::SourceClient;
use arvo_plugin_host::source::{serve, GrpcSource, Served};
use arvo_yfinance::Yahoo;

#[tokio::test]
async fn both_yahoo_venues_are_discovered_on_their_own_bases() {
    let addr: std::net::SocketAddr = "127.0.0.1:50081".parse().expect("an address");
    let plugin = Served::new(
        "yahoo",
        "Yahoo Finance",
        "0.0.0",
        vec![Box::new(Yahoo::new()), Box::new(Yahoo::total_return())],
    );
    tokio::spawn(serve(addr, plugin));
    for _ in 0..50 {
        if SourceClient::connect("http://127.0.0.1:50081".to_owned()).await.is_ok() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let found = GrpcSource::discover("http://127.0.0.1:50081").await.expect("discovered");
    let compiled: Vec<Box<dyn Source>> = vec![Box::new(Yahoo::new()), Box::new(Yahoo::total_return())];
    assert_eq!(found.len(), compiled.len());
    let five_minutes = BarInterval::new(5, IntervalUnit::Minute);
    for (served, own) in found.iter().zip(&compiled) {
        assert_eq!(served.id(), own.id());
        assert_eq!(served.venue(), own.venue());
        assert_eq!(served.basis(), own.basis(), "{}: the declaration the boundary exists to keep", own.id());
        assert_eq!(served.credential(), own.credential());
        assert_eq!(served.max_days(five_minutes), own.max_days(five_minutes));
        assert!(served.connected().await.expect("asked"), "Yahoo needs no credential");
    }
}
