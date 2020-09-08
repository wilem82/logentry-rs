pub type Zdt = chrono::DateTime<chrono::offset::Utc>;

pub struct LogEntry {
    pub text: String,
    pub zdt: Option<Zdt>,
}

pub struct LogEntryIterator<'a> {
    lines: Box<dyn Iterator<Item=String> + 'a>,

    entry_regex: &'a regex::Regex,
    timestamp_pattern: Option<&'a str>,

    entry_text: String,
    last_line: Option<String>,
}

impl<'a> LogEntryIterator<'a> {
    pub fn new(
        entry_regex: &'a regex::Regex,
        timestamp_pattern: Option<&'a str>,
        lines: Box<dyn Iterator<Item=String> + 'a>
    ) -> LogEntryIterator<'a> {
        LogEntryIterator {
            lines,
            entry_regex,
            timestamp_pattern,
            entry_text: String::new(),
            last_line: None,
        }
    }
}

impl<'a> Iterator for LogEntryIterator<'a> {
    type Item = LogEntry;

    fn next(&mut self) -> Option<LogEntry> {
        let lines_ref = &mut self.lines;
        let lines_ref_iter = std::iter::from_fn(|| lines_ref.next());
        let mut combined_lines: Box<dyn Iterator<Item=String>> = match &self.last_line {
            None => Box::new(lines_ref_iter),
            Some(_) => {
                let old = std::mem::replace(&mut self.last_line, None).unwrap();
                Box::new(std::iter::once(old).chain(lines_ref_iter))
            },
        };

        let mut zdt: Option<Zdt> = None;
        let entry_complete: bool = loop {
            let line = match combined_lines.next() {
                None => {
                    if self.entry_text.is_empty() {
                        break false
                    }
                    break true;
                },
                Some(x) => x,
            };
            let captured = self.entry_regex.captures(line.as_str());
            match captured {
                None => {
                    if self.entry_text.is_empty() {
                        // skip non-entry text before the first entry begins
                    } else {
                        self.entry_text.push_str("\n");
                        self.entry_text.push_str(line.as_str());
                    }
                },
                Some(x) => {
                    if self.entry_text.is_empty() {
                        zdt = match self.timestamp_pattern {
                            Some(it) => {
                                use chrono::offset::TimeZone;
                                Some(chrono::Utc.datetime_from_str(
                                    x.name("timestamp").unwrap().as_str(),
                                    it
                                ).unwrap())
                            },
                            None => None
                        };
                        self.entry_text.push_str(line.as_str());
                    } else {
                        self.last_line = Some(line);
                        break true;
                    }
                },
            }
        };
        if entry_complete {
            Some(LogEntry {
                text: std::mem::replace(&mut self.entry_text, String::new()),
                zdt: zdt,
            })
        } else {
            None
        }
    }
}
