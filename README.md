# Sliding window remote file copy algorithm

Prototypical implementation of an algorithmn that copies modified files from a source to a target assuming that an old version of the file is on the target and both the new and the old version of the file exists on the host.

## Build and run

- Compile: `cargo b`
- Run tests: `cargo t`
- Run main: `cargo r`

## Other helpful commands

- Add dependency: `cargo add <dependency>`
- Execute tests while printing output: `cargo t -- --nocapture`
- Format code: `cargo fmt`
- Check code style: `cargo fmt --all -- --check`