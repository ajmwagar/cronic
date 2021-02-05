use cronic::Scheduler;

#[tokio::main]
async fn main() {
    match Scheduler::new()
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
        .await
    {
        Ok(_) => {}
        Err(err) => eprintln!("An error occured: {}", err),
    }
}
