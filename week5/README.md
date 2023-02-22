# Weekly Rust Progress Report
Build a rust the domain of data engineering or machine learning engineering.


## Week 5 Progress
This week, I set up a frontend and server it with actix_web. 

```

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            Files::new("/", "./dist").index_file("index.html")
            ,
        )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

### Usage
> Go to week5 `cd week5`

> Run `cargo run` in the terminal, it will run the client.

```
go to browser and type http://localhost:8080
```
## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
