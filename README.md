# Epicenter

> Simple sync/async event dispatcher for Rust

[![crates.io](https://img.shields.io/crates/v/epicenter.svg)](https://crates.io/crates/epicenter)
[![download count badge](https://img.shields.io/crates/d/epicenter.svg)](https://crates.io/crates/epicenter)
[![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/epicenter)

## Usage

```rust
use epicenter::{Event, AsyncDispatcher};

#[derive(Debug, Clone)]
struct ExampleEvent {}
impl Event for ExampleEvent {}

let mut dispatcher = AsyncDispatcher::new();

dispatcher.listen(|event: &mut ExampleEvent| async move {
    // ...
}).await;

dispatcher.dispatch(ExampleEvent {}).await?;
```

Refer to the [documentation on docs.rs](https://docs.rs/epicenter) for detailed usage instructions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
