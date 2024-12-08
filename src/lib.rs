use std::path::{Path, PathBuf};
use std::thread;
use flexi_logger::{Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, FlexiLoggerError, Naming, Record};

fn custom_format(writer: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
    let file = match record.file() {
        None => {
            "<unknown>".to_string()
        }
        Some(path) => {
            Path::new(path).file_name().map(|v| v.to_string_lossy().to_string()).unwrap_or("<unknown>".to_string())
        }
    };
    write!(
        writer,
        "{} [{}] [{}:{}] [{}] - {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        file,
        record.line().unwrap_or(0),
        thread::current().name().unwrap_or("<unnamed>"),
        &record.args()
    )
}
pub struct Logger {
    log_level: String,
    log_to_file: bool,
    log_path: PathBuf,
    log_file_size: u64,
    log_file_count: usize,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            log_level: "debug".to_string(),
            log_to_file: false,
            log_path: std::env::current_dir().unwrap().join("logs"),
            log_file_size: 10 * 1024 * 1024,
            log_file_count: 7,
        }
    }

    pub fn set_log_level(&mut self, level: &str) {
        self.log_level = level.to_string();
    }

    pub fn set_log_to_file(&mut self, to_file: bool) {
        self.log_to_file = to_file;
    }

    pub fn set_log_path(&mut self, path: &str) {
        self.log_path = PathBuf::from(path);
    }

    pub fn set_log_file_size(&mut self, size: u64) {
        self.log_file_size = size;
    }

    pub fn set_log_file_count(&mut self, count: usize) {
        self.log_file_count = count;
    }

    pub fn start(&self) -> Result<(), FlexiLoggerError> {
        let mut logger = flexi_logger::Logger::try_with_str(self.log_level.as_str())?;
        if self.log_to_file {
            logger = logger.log_to_file(FileSpec::default().directory(self.log_path.as_path()))
                .duplicate_to_stderr(Duplicate::All)
                .rotate(Criterion::Size(self.log_file_size), // 文件大小达到 10MB 时轮转
                        Naming::Numbers, // 使用数字命名轮转文件
                        Cleanup::KeepLogFiles(self.log_file_count), // 保留最近 7 个日志文件
                );
        }
        logger.format(custom_format)
            .start()?;
        Ok(())
    }
}