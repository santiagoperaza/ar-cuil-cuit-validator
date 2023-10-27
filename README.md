# Simple Argentinian CUIL/CUIT validator

A command line tool and library to validate Argentinian CUIL/CUIT number.

The format of the CUIL/CUIT identifier consists of 10 digits plus a verification digit that is the result of an arithmetic operation based on the initial digits.

[Link to published crate on crates.io](https://crates.io/crates/ar_cuil_cuit_validator)

## Usage

It is required to provide the complete CUIL/CUIT without dashes.

We are using [clap](https://docs.rs/clap/latest/clap/) to parse the CLI arguments, so running `ar_cuil_cuit_validator -h` shows the expected format of the arguments.

### Examples:

#### Valid
```
ar_cuil_cuit_validator 27280335148

✅ The CUIL/CUIT 27280335148 is valid
```
#### Invalid
```
ar_cuil_cuit_validator 27280335141

❌ The CUIL/CUIT 27280335141 is not valid
```

#### Invalid format
```
ar_cuil_cuit_validator 27280

Invalid CUIL/CUIT format, it must be 11 digits, received: 27280
```

## Install

You can install the binary in your system with [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo install ar_cuil_cuit_validator
```

At this point `ar_cuil_cuit_validator` will be available as a binary in your system.

If you don't want to install it in your system you can run it with cargo from the project directory.

```
cargo run 27280335148

✅ The CUIL/CUIT 27280335148 is valid
```

## Programmatic usage

Run the following Cargo command in your project directory:

```cargo add ar_cuil_cuit_validator```

Or add the following line to your Cargo.toml:

```toml
[dependencies]
ar_cuil_cuit_validator = "*"
```

Then use it in your code

```rust
use ar_cuil_cuit_validator::cuil_cuit;

let valid_cuil_cuit = 27280335148;
let result = is_valid(valid_cuil_cuit).unwrap();
assert_eq!(true, result);
```

### Unit Tests

Execute `cargo test`