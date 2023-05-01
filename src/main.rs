use sha2::{Digest, Sha256};
use std::collections::HashMap;

struct ChunkContainer {
    chunks: Vec<Chunk>,
}

#[derive(Debug, Clone)]
struct Chunk {
    size: usize,
    position: usize,
    hash: String,
}

impl ChunkContainer {
    fn hash_to_chunk(&self) -> HashMap<String, Chunk> {
        let mut map = HashMap::new();
        for chunk in &self.chunks {
            // TODO GJA there is cloning going on here
            // this is probably expensive. What is the usual
            // way to make this work?
            map.insert(chunk.hash.clone(), chunk.clone());
        }
        map
    }
}

#[derive(Debug, PartialEq, Eq)]
enum OperationType {
    TRANSFER,
    COPY,
}

#[derive(Debug)]
struct Operation {
    operation_type: OperationType,
    start: usize,
    bytes: usize,
    // The payload is only filled if the operation type is transfer, meaning that
    // it has to be transferred from the source to the target because we know that
    // this window from position start with length bytes is not existing on the
    // target machine.
    payload: Vec<u8>,
}

fn sliding_window_analyze(old: &Vec<u8>, new: &Vec<u8>, window_size: usize) -> Vec<Operation> {
    // println!("Old: {}", old);
    // println!("New: {}", new);

    // println!("Window size: {}", window_size);

    // let mut byte_number = 0;

    // // println!("\nSource:");
    // for b in old.bytes() {
    //     let char = b as char;
    //     println!("byte\t{}: \t{}\t(char {})", byte_number, b, char);
    //     byte_number += 1;
    // }

    // println!("\nold windows:");
    let old_container = windows(&old, window_size);
    let old_hash_to_chunk = old_container.hash_to_chunk();

    // println!("\new windows:");
    let new_container = windows(&new, window_size);

    // println!("\nDiffing...");
    // Wir wollen wissen welche chunks des target containers bereits im source container sind
    // bzw. welche stellen des source container wo im target container eingetragen werden muessen (Die Luecken)

    let mut last_position: usize = 0;
    let mut operations: Vec<Operation> = Vec::new();

    for chunk in new_container.chunks {
        let position_is_already_covered = chunk.position < last_position;

        if position_is_already_covered {
            continue;
        }

        let old_chunk = old_hash_to_chunk.get(&chunk.hash);
        let found = old_chunk.is_some();
        // println!(
        //     "chunk {}: {} \t[{};{}[",
        //     chunk.hash,
        //     found,
        //     chunk.start_position(),
        //     chunk.end_position()
        // );

        let operation = match found {
            true => Operation {
                operation_type: OperationType::COPY,
                start: old_chunk.unwrap().position,
                bytes: chunk.size,
                payload: [].to_vec(),
            },
            false => Operation {
                operation_type: OperationType::TRANSFER,
                start: chunk.position,
                bytes: chunk.size,
                payload: new[chunk.position..chunk.position + chunk.size].to_vec(),
            },
        };
        operations.push(operation);
        last_position += window_size;
    }

    // Handle the last chunk if it is smaller then the window size
    if last_position < new.len() {
        operations.push(Operation {
            operation_type: OperationType::TRANSFER,
            start: last_position,
            bytes: new.len() - last_position,
            payload: new[last_position..new.len()].to_vec(),
        });
    }

    operations
}

fn sliding_window_restore(old: &Vec<u8>, operations: Vec<Operation>) -> Vec<u8> {
    // perform this has to be implemented in the target.
    // The interface is the list of operations.
    let mut result: Vec<u8> = Vec::new();
    for operation in &operations {
        let mut chunk: Vec<u8> = match operation.operation_type {
            OperationType::COPY => &old[operation.start..operation.start + operation.bytes],
            OperationType::TRANSFER => operation.payload.as_slice(),
        }
        .to_vec();
        result.append(&mut chunk);
    }
    result
}

fn sha256(window_bytes: &[u8]) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(window_bytes);
    let hash = sha256.finalize();
    format!("{:X}", hash)
}

fn windows(source: &Vec<u8>, size: usize) -> ChunkContainer {
    ChunkContainer {
        chunks: source
            .windows(size)
            .enumerate()
            .map(|(i, window)| {
                let hash = sha256(window);
                // println!("window\t{}: \t{:?} (sha256: {})", i, window, hash);
                Chunk {
                    size: size,
                    position: i,
                    hash: hash,
                }
            })
            .collect(),
    }
}

fn main() {
    let old = &String::from("guilllero").bytes().collect();
    let new = &String::from("guillermo").bytes().collect();
    let window_size = 2;
    let operations = sliding_window_analyze(old, new, window_size);
    let restored = sliding_window_restore(old, operations);
    println!("restored: {}", String::from_utf8(restored).unwrap());
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read};

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    #[case(6)]
    fn string_window_size_test(#[case] window_size: usize) {
        println!("window_size: {}", window_size);
        let expected = "guillermo".bytes().collect::<Vec<_>>();
        let old = "guilllero".bytes().collect();
        let new = "guillermo".bytes().collect();
        let operations = sliding_window_analyze(&old, &new, window_size);
        let restored = sliding_window_restore(&old, operations);
        assert_eq!(expected, restored);
    }

    #[rstest]
    fn file_tests() {
        let window_size = 128;
        let lorem1 = read_bytes_from_file("lorem.txt");
        let lorem2 = read_bytes_from_file("lorem2.txt");
        let operations = sliding_window_analyze(&lorem1, &lorem2, window_size);
        let restored = sliding_window_restore(&lorem1, operations);
        assert_eq!(lorem2, restored);
    }

    fn read_bytes_from_file(path: &str) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        File::open(path).unwrap().read_to_end(&mut buf).unwrap();
        buf
    }
}
