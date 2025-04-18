use tokio::time::{sleep, Duration};
use std::error::Error;

#[derive(Debug)]
struct TaskError(String);

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TaskError {}

async fn process_data(id: u32, processing_time: u64) -> Result<String, TaskError> {
    println!("Processing data for task {}", id);
    tokio::select! {
        _ = sleep(Duration::from_secs(processing_time)) => {
            if processing_time > 3 {
                return Err(TaskError(format!("Processing timeout for task {} after {} seconds", id, processing_time)));
            }
            Ok(format!("Data processed for task {}", id))
        }
        _ = sleep(Duration::from_secs(5)) => {
            Err(TaskError(format!("Task {} exceeded maximum processing time of 5 seconds", id)))
        }
    }
}

async fn network_request(id: u32) -> Result<(), TaskError> {
    println!("Simulating network request for task {}", id);
    tokio::select! {
        _ = sleep(Duration::from_millis(500)) => {
            Ok(())
        }
        _ = sleep(Duration::from_secs(1)) => {
            Err(TaskError(format!("Network request timeout for task {}", id)))
        }
    }
}

async fn complex_task(id: u32, processing_time: u64) -> Result<String, TaskError> {
    let timeout = Duration::from_secs(6); // Overall timeout for the entire task
    tokio::time::timeout(timeout, async {
        network_request(id).await?;
        process_data(id, processing_time).await
    })
    .await
    .map_err(|_| TaskError(format!("Task {} timed out after {} seconds", id, timeout.as_secs())))?
}

#[tokio::main]
async fn main() {
    println!("Starting complex async tasks...");

    let tasks = vec![
        complex_task(1, 2),
        complex_task(2, 1),
        complex_task(3, 4),
    ];

    let results = futures::future::join_all(tasks).await;
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(output) => println!("Task {} completed successfully: {}", i + 1, output),
            Err(e) => println!("Task {} failed: {}", i + 1, e),
        }
    }

    println!("All tasks completed!");
}
