use chrono::{TimeZone, Utc};
use cron::Schedule;
use futures_util::future::join_all;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use tokio::time::sleep;

type JobOutput = Result<(), Box<dyn Error>>;

/// Cronic Scheduler
pub struct Scheduler<C: Sized + Default + Clone, F: Future<Output = T> + Send + 'static, T: Send + 'static> {
    context: Option<C>,
    jobs: Vec<(String, Box<dyn Fn(C) -> F>)>
}

impl<'a, C: Default + Clone, F: Future<Output = T> + Send, T: Send> Scheduler<C, F, T> {
    pub fn new() -> Self {
        Self {
            context: None,
            jobs: Vec::new(),
        }
    }

    pub fn job<S: Into<String>>(mut self, cron_str: S, job: impl Fn(C) -> F + 'static) -> Self
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.jobs.push((cron_str.into(), Box::new(job)));
        self
    }

    pub fn set_context(mut self, context: C) -> Self {
        self.context = Some(context);
        self
    }

    /// Start the execution of the scheduled jobs
    pub async fn start(self) -> Result<(), Box<dyn Error>> {
        // Set up schedules

        let context = self.context.unwrap_or_default();

        let mut futures = Vec::new();

        for (cron, job) in self.jobs {
            let con = context.clone();
            let schedule = Schedule::from_str(&cron)?;

            let fut = async move {
                for datetime in schedule.upcoming(Utc) {
                    let now = Utc::now();

                    if let Ok(duration) = datetime.signed_duration_since(now).to_std() {
                        sleep(duration).await;
                        let _result = job(con.clone()).await;
                    }
                }
            };

            futures.push(fut);
        }

        join_all(futures).await;

        Ok(())
    }
}
