fn main() {
    sfo_log::Logger::new("test").set_output_to_console(true).set_log_to_file(false).set_log_level("debug").start().unwrap();
    sfo_log::error!("error");
    sfo_log::debug!("debug");
    sfo_log::info!("debug");
    sfo_log::trace!("trace");
    sfo_log::warn!("warning");
}
