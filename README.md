# RLP Serde

A lightweight and idiomatic Rust crate for **RLP (Recursive Length Prefix)** serialization and deserialization using the [`serde`](https://serde.rs/) framework.

## Features

- ✅ Serde-compatible RLP encode/decode  
- ✅ Works with structs, enums, sequences  
- ✅ Minimal and efficient implementation  
- ⚠️ **Nested array serialization and deserialization are not supported yet**

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde-rlp = { git = "https://github.com/athulr32/serde-rlp" }
serde = { version = "1.0", features = ["derive"] }
```

## Example

```rust
use serde::{Serialize, Deserialize};
use serde_rlp::{to_rlp_bytes, from_rlp_bytes};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: Vec<String>,
}

fn main() {
    let point = Point {
        x: vec![String::from("cat"), String::from("dog")],
    };

    // Serialize to RLP bytes
    let bytes = to_rlp_bytes(&point).expect("serialization failed");
    println!("Encoded: {:?}", bytes);

    // Deserialize back to struct
    let decoded: Point = from_rlp_bytes(&bytes).expect("deserialization failed");
    println!("Decoded: {:?}", decoded);
}
```

## API

### `to_rlp_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, RlpError>`

Serializes a Rust data structure into RLP bytes.

### `from_rlp_bytes<T: DeserializeOwned>(input: &[u8]) -> Result<T, RlpError>`

Deserializes RLP bytes into a Rust data structure.

## Limitations

- ❌ Nested array/sequence **serialization and deserialization** are not supported yet

## Testing

Run the test suite with:

```bash
cargo test
```

## License

Licensed under the [MIT License](LICENSE).

---

