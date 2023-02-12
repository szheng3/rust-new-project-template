# Weekly Rust Progress Report
Build a rust the domain of data engineering or machine learning engineering.


## Week 4 Progress
This week, I set up a rust client in order to call the API server for the price of the cryptocurrency. 

```

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("https://data.binance.com/api/v3/ticker/24hr")
        .await?
        .json::<Vec<Ticker>>()
        .await?;
    println!("{:#?}", resp[0]);
    Ok(())
}

```

### Usage
> Go to week4 `cd week4`

> Run `cargo run` in the terminal, it will run the client.

```
Ticker {
    symbol: "ETHBTC",
    price_change: "0.00000000",
    price_change_percent: "0.000",
    weighted_avg_price: "0.03271000",
    prev_close_price: "0.03271000",
    last_price: "0.03271000",
    last_qty: "0.00000000",
    bid_price: "0.03271000",
    bid_qty: "0.00000000",
    ask_price: "0.03272000",
    ask_qty: "0.00000000",
    open_price: "0.03271000",
    high_price: "0.03271000",
    low_price: "0.03271000",
    volume: "0.00000000",
    quote_volume: "0.00000000",
    open_time: 1620000000000,
    close_time: 1620000000000,
    first_id: 0,
    last_id: 0,
    count: 0,
}
```
## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
