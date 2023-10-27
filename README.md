# Simple Argentinian CUIL/CUIT validator

CLI tool to validate Argentinian CUIL/CUIT number.

The format of the CUIL/CUIT identifier consists of 10 digits plus a verification digit that is the result of an arithmetic operation based on the initial digits.

## Instructions

It is required to provide the complete CUIL/CUIT without dashes.

We are using [clap](https://docs.rs/clap/latest/clap/) to parse the CLI arguments, so running `cargo run -- -h` shows the expected format of the arguments.

### Examples:

#### Valid
```
cargo run -- 27280335148

✅ The CUIL/CUIT 27280335148 is valid
```
#### Invalid
```
cargo run -- 27280335141

❌ The CUIL/CUIT 27280335141 is not valid
```

#### Invalid format
```
cargo run -- 27280

Invalid CUIL/CUIT format, it must be 11 digits, received: 27280
```

### Unit Tests

Execute `cargo test`