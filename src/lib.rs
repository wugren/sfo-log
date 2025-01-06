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
        thread::current().name().unwrap_or(format!("{:?}", thread::current().id()).as_str()),
        &record.args()
    )
}
pub struct Logger {
    app_name: String,
    log_level: String,
    log_to_file: bool,
    log_path: PathBuf,
    log_file_size: u64,
    log_file_count: usize,
    instance_id: String,
    output_console: bool,
}

impl Logger {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            log_level: "debug".to_string(),
            log_to_file: false,
            log_path: std::env::current_dir().unwrap().join("logs"),
            log_file_size: 10 * 1024 * 1024,
            log_file_count: 7,
            instance_id: "".to_string(),
            output_console: true,
        }
    }

    pub fn set_instance_id(mut self, instance_id: &str) -> Self {
        self.instance_id = instance_id.to_string();
        self
    }

    pub fn set_process_id_to_instance_id(mut self) -> Self {
        self.instance_id = format!("{}", std::process::id());
        self
    }

    pub fn set_log_level(mut self, level: &str) -> Self {
        self.log_level = level.to_string();
        self
    }

    pub fn set_log_to_file(mut self, to_file: bool) -> Self {
        self.log_to_file = to_file;
        self
    }

    pub fn set_log_path(mut self, path: &str) -> Self {
        self.log_path = PathBuf::from(path);
        self
    }

    pub fn set_log_file_size(mut self, size: u64) -> Self {
        self.log_file_size = size;
        self
    }

    pub fn set_log_file_count(mut self, count: usize) -> Self {
        self.log_file_count = count;
        self
    }

    pub fn set_output_to_console(mut self, output_console: bool) -> Self {
        self.output_console = output_console;
        self
    }

    pub fn start(&self) -> Result<(), FlexiLoggerError> {
        let mut logger = flexi_logger::Logger::try_with_env_or_str(self.log_level.as_str())?;
        if self.log_to_file {
            let mut base_name = self.app_name.clone();
            if !self.instance_id.is_empty() {
                base_name = format!("{}_{}", self.app_name, self.instance_id);
            }
            logger = logger.log_to_file(FileSpec::default().directory(self.log_path.as_path()).basename(base_name.as_str()))
                .rotate(Criterion::Size(self.log_file_size), // 文件大小达到 10MB 时轮转
                        Naming::Numbers, // 使用数字命名轮转文件
                        Cleanup::KeepLogFiles(self.log_file_count), // 保留最近 7 个日志文件
                );
        }
        if !self.output_console {
            logger = logger.duplicate_to_stderr(Duplicate::None);
        } else {
            logger = logger.duplicate_to_stderr(Duplicate::All);
        }
        logger.format(custom_format)
            .start()?;
        Ok(())
    }
}
