use cronic::Scheduler;

#[tokio::main]
async fn main() {
    match Scheduler::new()
        .set_context(())
        .job("* * * * * *", |_| async {
            println!("Every second!");
        })
        .job("*/5 * * * * *", |_| async {
            println!("Every five seconds!");
        })
        .job("*/5 * * * * *", |_| async {
            println!("Every five seconds!");
        })
    .start().await {
        Ok(_) => {},
        Err(err) => eprintln!("An error occured: {}", err)
    }
}
