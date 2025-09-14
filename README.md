# Logger

**Developer:** Urban Egor  
**Version:** 1.3.14 r  

A simple multithreaded logger for Rust. It writes log messages to the terminal and to daily log files.

---

## Features

- Logs messages with different levels: `DEBUG`, `INFO`, `WARN`, `ERROR`.
- Prints logs to the terminal.
- Saves logs to daily files in the `logs/` directory.
- Thread-safe, non-blocking logging using channels.
- Easy-to-use global logger instance.

---

## How It Works

1. The logger creates a `logs/` folder if it does not exist.
2. A background thread listens for log messages sent via a channel.
3. Each log message is printed to the terminal and appended to a file named by the current date, e.g., `2025-09-15.log`.
4. Log messages include the timestamp, log level, and the message text.

Example log message:

[2025-09-15 14:23:01][INFO] Server started successfully


---


The LOGGER is a global static instance, so you don't need to create it manually.

Log Levels:

> DEBUG – detailed information for debugging.
> 
> INFO – general information about program execution.
> 
> WARN – warnings, something might be wrong.
> 
> ERROR – errors that need attention.


## Installation:

Add this logger to your project by copying the source file to your project and importing it where needed.


---


## Notes:

Make sure your application has write permission to create the logs/ folder.

Logging is asynchronous, so messages are sent to the logging thread immediately.

License: MIT (or your preferred license)
