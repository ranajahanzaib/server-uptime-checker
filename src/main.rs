use tokio_cron_scheduler::{JobScheduler, Job};

mod config;   
mod checker;  
mod notifier; 

/// Entry point of the application. Configures and starts the job scheduler.
///
/// The scheduler periodically checks the status of servers defined in the configuration file
/// and sends email notifications if a server is down.
#[tokio::main]
async fn main() {
    // Load the configuration from the specified JSON file.
    let config = config::load_config("config/servers.json");

    // Create a new job scheduler.
    let scheduler = JobScheduler::new().await.unwrap();

    // Iterate over the configured servers and add a job for each server.
    for server in config.servers {
        let url = server.url.clone();
        let method = server.method.clone();

        // Schedule a job to check the server's status every second.
        scheduler
            .add(Job::new_async("*/1 * * * * *", move |_, _| {
                let url = url.clone();
                let method = method.clone();

                Box::pin(async move {
                    // Determine the appropriate check based on the method.
                    let is_up = match method.as_str() {
                        "http" => checker::check_http(&url).await,
                        "ping" => checker::check_ping(&url),
                        _ => false,
                    };

                    // If the server is down, send an email notification.
                    if !is_up {
                        notifier::send_email(
                            "user@example.com",
                            "Server Down",
                            &format!("Server {} is down", url),
                        );
                    }
                })
            })
            .unwrap())
            .await
            .unwrap();
    }

    // Start the job scheduler.
    scheduler.start().await.unwrap();

    // Wait for a Ctrl+C signal to gracefully shut down.
    tokio::signal::ctrl_c().await.unwrap();
}
