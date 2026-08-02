use std::fmt::{self, Display};

#[derive(Debug)]
pub struct LevelCounter {
    pub info_count: usize,
    pub warn_count: usize,
    pub error_count: usize,
}

impl LevelCounter {
    pub fn new() -> Self {
        Self {
            info_count: 0,
            warn_count: 0,
            error_count: 0,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "INFO: {}\nWARN: {}\nERROR: {}",
            self.info_count, self.warn_count, self.error_count
        )
    }
}

#[derive(Debug)]
pub struct UnknownLevel {
    pub index: usize,
    pub level: String,
}

impl UnknownLevel {
    pub fn new(index: usize, level: String) -> Self {
        Self { index, level }
    }

    pub fn display(&self) -> String {
        format!("log #{}: {}", self.index, self.level)
    }
}

#[derive(Debug)]
pub struct UnknownLevelError {
    logs: Vec<UnknownLevel>,
}

impl UnknownLevelError {
    fn new() -> Self {
        Self { logs: vec![] }
    }

    fn append(&mut self, unknown: (&usize, &str)) {
        self.logs
            .push(UnknownLevel::new(*unknown.0, String::from(unknown.1)));
    }
}

impl Display for UnknownLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Unknown log levels\n***")?;
        for item in self.logs.iter() {
            writeln!(f, "{}", item.display())?;
        }
        write!(f, "***\nAllowed log levels:\n  INFO, WARN, ERROR")?;
        Ok(())
    }
}

impl std::error::Error for UnknownLevelError {}

fn split_first_space(s: &str) -> Option<(&str, &str)> {
    if s.trim().is_empty() {
        None
    } else {
        if let Some(i) = s.find(' ') {
            Some((&s[..i], &s[i + 1..]))
        } else {
            Some((s, ""))
        }
    }
}

pub fn aggregate(logs: &[String]) -> Result<LevelCounter, UnknownLevelError> {
    let mut counter = LevelCounter::new();
    let mut error = UnknownLevelError::new();

    for (i, log) in logs.iter().enumerate() {
        let splited = split_first_space(log);
        match splited {
            Some((level, _message)) => match level {
                "INFO" => counter.info_count += 1,
                "WARN" => counter.warn_count += 1,
                "ERROR" => counter.error_count += 1,

                _ => error.append((&(i + 1), level)),
            },
            None => continue,
        }
    }

    if error.logs.is_empty() {
        Ok(counter)
    } else {
        Err(error)
    }
}
