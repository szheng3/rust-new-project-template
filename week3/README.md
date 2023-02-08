# Weekly Rust Progress Report
Build a rust the domain of data engineering or machine learning engineering.


## Week 3 Progress
This week, I set up a rust bench. In order to test the performance of the Rust, I write a single fibonacci sequence. The result is shown below.

```
pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

### Usage
> Go to week3 `cd week3`

> Run `make bench` in the terminal, it will run the benchmark test for fibonacci.

## Benchmark Results
![Benchmark](./assets/fb.png)

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [server](https://codevoweb.com/build-a-simple-api-with-rust-and-actix-web/)