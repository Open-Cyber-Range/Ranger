## Ranger

This is an execution runtime project for the cyber range. It's main purpose is to coordinate deployment of SDL-based scenario in live environment.

### Developing

Use VSCode and [devcontainers](https://code.visualstudio.com/docs/remote/containers) to develop ranger and automatically have its dependencies deployed.

Use `cargo install cargo-insta` to have access to snapshot tooling under `cargo insta` command.

Use `cargo install diesel-cli` to have access to database management (setup, migrations, etc.) under `diesel` command.

#### Setup

Before opening the folder in `devcontainer` configurations for dependency services need to be filled out. To get the apporiate list of configurations consult either `.gitignore` or `.devcontainer/docker-compose.yml` files.

