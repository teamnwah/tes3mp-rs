# tes3mp-plugin

A crate for helping you make tes3mp plugins in Rust

# How to use

Because Rust won't allow you to re-export symbols from other crates, this crate can't be used like normal crates, the advised manner of using this crate is the following:

```bash
cd crate-dir
git status || git init
mkdir extern
git submodule add https://git.cijber.net/teamnwah/tes3mp-rs.git extern/tes3mp-rs
ln -s extern/tes3mp-rs/tes3mp-plugin/src/plugin src/plugin
```

This will make the `plugin` like it's part of your crate, which allows us to export the C symbols

in your `Cargo.toml` make sure the following is included

```toml
[lib]
crate-type = ["staticlib", "cdylib"]
```

This will make sure you get a shared library which can be loaded by TES3MP

Using the code is now rather simple, write in your lib.rs

```rust
use crate::plugin::Events;

mod plugin;

struct Server;

impl Events for Server {
    fn new() -> Self {
        Server
    }

    fn on_server_init(&self) {
        plugin::log_message(plugin::LOG_WARN, "Hello from Rust :3");
    }
}

use_events!(Server);
```

the `Events` trait has all the events that exist for the server, implement as needed