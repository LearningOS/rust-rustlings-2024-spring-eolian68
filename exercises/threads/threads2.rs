// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            //status_shared.jobs_completed += 1;

            // 方式 1 ：
            // let mut num = status_shared.lock().unwrap();
            // (*num).jobs_completed += 1;

            // 方式 2 ：
            // (*status_shared.lock().unwrap()).jobs_completed += 1;

            // 方式 3 ：JobStatus 未实现 Deref trait 但对其自身字段可自动解引用[E?]
            status_shared.lock().unwrap().jobs_completed += 1;

        });
        handles.push(handle);
    }
    for handle in handles {
        // 当注释掉 handle.join().unwrap(); 后亦会等待所有线程结束后退出，但以下的输出全是 0
        handle.join().unwrap();

        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
