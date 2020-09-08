use crate::entry::*;

pub struct MultiLogEntryIterator<'a> {
    iters_and_values: Vec<IterValue<'a>>,
}

pub struct LogEntryIteratorWithSource<'a> {
    iter: LogEntryIterator<'a>,
    source: Option<String>,
}

impl<'a> LogEntryIteratorWithSource<'a> {
    pub fn new(iter: LogEntryIterator<'a>, source: Option<String>) -> LogEntryIteratorWithSource<'a> {
        LogEntryIteratorWithSource {
            iter,
            source
        }
    }
}

impl<'a> MultiLogEntryIterator<'a> {
    pub fn new(mut entry_iterators: Vec<LogEntryIteratorWithSource>) -> MultiLogEntryIterator {
        let total_iterators = entry_iterators.len();
        MultiLogEntryIterator {
            iters_and_values: {
                let mut vec = Vec::<IterValue>::with_capacity(total_iterators);
                for _ in 0..total_iterators {
                    let mut iter = entry_iterators.pop().unwrap();
                    let value = iter.iter.next();
                    vec.push(IterValue {
                        iter: iter.iter,
                        source: iter.source,
                        value: value,
                        finished: false,
                    });
                }
                vec
            },
        }
    }
}

struct IterValue<'a> {
    iter: LogEntryIterator<'a>,
    source: Option<String>,
    value: Option<LogEntry>,
    finished: bool,
}

impl<'a> Iterator for MultiLogEntryIterator<'a> {
    type Item = (LogEntry, Option<String>);

    fn next(&mut self) -> Option<(LogEntry, Option<String>)> {
        if self.iters_and_values.is_empty() {
            return None;
        }

        for n in 0..self.iters_and_values.len() {
            let mut iter_value = &mut self.iters_and_values[n];
            if iter_value.value.is_some() {
                continue;
            }
            let new_value = iter_value.iter.next();
            if new_value.is_none() {
                iter_value.finished = true;
                continue;
            }
            iter_value.value = new_value;
        }

        self.iters_and_values.retain(|it| !it.finished);

        if self.iters_and_values.is_empty() {
            return None;
        }

        let min = self.iters_and_values.iter_mut().min_by(|a, b| {
            let a_zdt = a.value.as_ref().unwrap().zdt.unwrap();
            let b_zdt = b.value.as_ref().unwrap().zdt.unwrap();
            a_zdt.cmp(&b_zdt)
        }).unwrap();
        let value_opt = std::mem::replace(&mut min.value, None).unwrap();
        Some((value_opt, min.source.clone()))
    }
}
