# Sliding window remote file copy algorithm / protocol

[![Rust](https://github.com/guija/sliding-window-copy-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/guija/sliding-window-copy-rust/actions/workflows/rust.yml)

Prototypical implementation (in Rust) of an algorithm / protocol that copies modified files from a source to a target assuming that an old version of the file is on the target and both the new and the old version of the file exists on the host.

_This project was implement for the sake of learning the programming language Rust._

## Use cases

This algorithm might be used in build systems like [just](https://github.com/just-buildsystem/justbuild) or [bazel](https://bazel.build/) to avoid copying the rebuilt artifacts/binary to the target machine as a whole.

## Description

### Principal idea

Given a file `a` which was copied from a source machine to a target machine. Then the file `a` is modified resulting in `a1`. Now both `a` and `a1` are available on source and `a` is available at target. The idea is to analyse the differences and common parts between `a` and `a1` on the source and transfer those differences in order to reconstruct `a1` on the target.

The protocol is divided into three parts:
- Analyse differences and created patches
- Transfer patches
- Reconstruct target file by copying existing parts and applying patches

### Analyse differences and created patches

 First, analyse the differences between `a` and `a1`. This is done by sliding windows over `a` and `a1`. For each window a hash is calculated. Then we find all the windows in `a1` that are already existing in `a`. For all bytes that are not covered by such a window we create a patch, called `Operation.TRANSFER`. For all windows that are already covered we created an `Operation.COPY`. `TRANSFER` means that the actual bytes have to be copied from the source to the target. `COPY` means that the bytes can be locally copied on the target from as we know they are already present there. 

 The signature is the following:

 ```rust
 fn sliding_window_analyze(old: &Vec<u8>, new: &Vec<u8>, window_size: usize) -> Vec<Operation>
 ```

 ### Transfer patches

 The set of `Operations` can be transferred from the source to the target machine. Both `TRANSFER` and `COPY` operations are transferred while online the `TRANSFER` operations contain a binary payload whereas the `COPY` operations only carry meta data.

 ### Reconstruct target file by copying existing parts and applying patches

 Now a new file `a1` is created on the target machine. All `Operations` are now applied. Either we copy a part of the original file `a` or we copy the patch that we received from the source. Combining the operations restores the modified file `a1`.

```rust
fn sliding_window_restore(old: &Vec<u8>, operations: Vec<Operation>) -> Vec<u8>
```

## Improvement ideas

### Identification of common windows

Currently a simple heuristic is being chosen in order to find windows that already exist in the source file. That is that the windows of the modified file are iterated on and then checked if it the window is already existing in the original file. This may not result in the optimal number of patches. A better approach would be to minimize the number of windows to be transferred treating it as optimisation problem by finding the windows that cover the largest possible part of the modified file while not intersecting those windows.

It may also be possible to reuse windows more than once. Currently even if the hash of a window is the same in different parts of the file they will be transferred multiple times.

### Avoiding to transfer `COPY` operations.
Currently the meta data of `COPY` operations are transferred between the analysis and the restoration part. However this is not necessary as by knowing which parts have to be transferred we can deduce which parts have to be copied. This works as long as the `TRANSFER` operations are not overlapping. Implementing this will reduce the amount of metadata that needs to be transferred.

## Build and run

- Compile: `cargo b`
- Run tests: `cargo t`
- Run main: `cargo r`

## Other helpful commands

- Add dependency: `cargo add <dependency>`
- Execute tests while printing output: `cargo t -- --nocapture`
- Format code: `cargo fmt`
- Check code style: `cargo fmt --all -- --check`