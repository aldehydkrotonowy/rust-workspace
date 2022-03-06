1. add new workspace
   cargo new some_name [--lib]
2. edit cargo.toml

A. watch and rebuild in workspace
cargo watch -x "run --bin rocket_web_server"
cargo watch -x "run -p rocket_web_server"
