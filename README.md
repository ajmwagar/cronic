# cronic

A cron-enabled task scheduler for `async` Rust

```toml
[dependencies]
cronic = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust
use cronic::Scheduler;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Scheduler::new()
        .set_context(())
        .job("@hourly", &|_| {
            Box::pin(async {
                println!("Every hour!");
            })
        })
        .job("* * * * * *", &|_| {
            Box::pin(async {
                println!("Every second!");
            })
        })
        .job("0 * * * * *", &|_| {
            Box::pin(async {
                println!("Every minute!");
            })
        })
        .start()
        .await?;

    Ok(())
}
```
