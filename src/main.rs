extern crate job_scheduler;

use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

fn main() {
    let mut scheduler = JobScheduler::new();

    // Adding a task to scheduler to execute it in every 2 minutes.
    scheduler.add(Job::new("1/2 * * * * *".parse().unwrap(), || {
        println!("Get executed every 2 seconds!");
    }));

    // Adding a task to scheduler to execute it in every 2 minutes.
    scheduler.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("Get executed every 10 seconds!");
    }));

    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_millis(100));
    }
}
