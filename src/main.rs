use habit_tracker::configuration::get_configuration;
use habit_tracker::startup::Application;
use habit_tracker::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("habit_tracker_service".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
