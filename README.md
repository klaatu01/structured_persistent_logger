# Structured Persistent Logger

`structured_persistent_logger` is a Rust crate providing a structured logging solution that is compatible with the standard `log` crate. It features persistent logging capabilities, allowing for the retention and structured formatting of log records.

## Features

- Integration with the standard `log` crate.
- Structured logging with JSON formatting.
- Persistent logging capabilities across application lifecycle.
- Thread-safe logging with support for multi-threaded environments.

## Installation

Add `structured_persistent_logger` to your `Cargo.toml`:

```toml
[dependencies]
structured_persistent_logger = "0.1.0"
log = "0.4.0"
```

## Usage

To use `structured_persistent_logger`, initialize it at the start of your application:

```rust
use structured_persistent_logger::StructuredPersistentLogger;

fn main() {
    StructuredPersistentLogger::init();
    add_persistent_logs! {
        "key1" => json!("value1"),
        "key2" => json!({"nested": "structure"})
    };
    log::info!("This is an info message");
    // output:
    // {
    //    "key1": "value1",
    //    "key2": { "nested": "structure" },
    //    "message": "This is an info message",
    //    "level": "INFO"
    // }
}
```

### Adding Persistent Logs

You can add persistent logs that will be included in every log message:

```rust
use structured_persistent_logger::add_persistent_logs;
use serde_json::json;

fn some_function() {
    add_persistent_logs! {
        "key1" => json!("value1"),
        "key2" => json!(123)
    };
}
```

## Configuration

The logger can be configured to adjust its behavior, such as setting log levels or customizing the format of log entries.

## Contributing

We welcome contributions to `structured_persistent_logger`! Please feel free to submit pull requests or open issues to improve the library.

## License

This crate is licensed under [MIT license](LICENSE).
