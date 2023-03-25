use flexi_logger::{AdaptiveFormat, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
use server::run_server;

pub fn initialize_logging() {
    Logger::try_with_env_or_str("debug")
        .expect("Couldn't initialize logger object")
        .log_to_file(FileSpec::default().directory("log").suppress_timestamp())
        .rotate(
            // Keep 10 MB log files
            Criterion::Size(10u64 * (1000 * 1000)),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(20),
        )
        .adaptive_format_for_stderr(AdaptiveFormat::Default)
        .append()
        .duplicate_to_stderr(Duplicate::Debug)
        .start()
        .expect("Couldn't start initialized logger");
}

#[tokio::main]
async fn main() {
    initialize_logging();
    run_server().await;
}
