# Summary

Regex-based iterators over log files.

The key point is a log entry can span multiple lines, making line-by-line search tools like `grep` or `rg` less than ideal.

All iterators are memory-effcient in the sense that a constant-size buffer is used for reading and parsing of the text.

# logentry::entry::LogEntryIterator

Takes a text input and produces `LogEntry` structs in the order they appear at source. A single log entry contains all of the message content and, if a timestamp format was provided, the message's parsed timestamp.

# logentry::multi::LogEntryIteratorWithSource

Like `LogEntryIterator`, but can take a textual description of where the data came from. Useful for keeping track of source file names.

# logentry::multi::MultiLogEntryIterator

Takes multiple `LogEntryIteratorWithSource` and produces `LogEntry` structs in chronological order, as long as the inputs were already (individually) sorted.

Can optionally prepend each output log entry with its source, eg. a file name.

# Examples

See the https://github.com/wilem82/logtools-rs source code, which are multiple CLI tools based on `logentry`.
