// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{ops::Deref, sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // DONE: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // DONE: You must take an action before you update a shared value.
            status_shared.deref().lock().unwrap().jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // DONE: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.deref().lock().unwrap().jobs_done);
}
