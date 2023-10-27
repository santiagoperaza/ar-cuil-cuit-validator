mod cuil_cuit;

use ar_cuil_cuit_validator::cuil_cuit::CuilCuitError;
use clap::Parser;

#[derive(Parser, Debug)]
struct CliArgs {
    /// Required to be 11 digits, only numbers, without dashes
    #[clap(verbatim_doc_comment)]
    cuil_cuit: u64,
}

fn main() {
    let CliArgs { cuil_cuit } = CliArgs::parse();
    let is_valid = cuil_cuit::is_valid(cuil_cuit)
        .map_err(|_e| CuilCuitError::InvalidFormat(cuil_cuit.to_string()));
    match is_valid {
        Ok(true) => println!("✅ The CUIL/CUIT {} is valid", cuil_cuit),
        Ok(false) => println!("❌ The CUIL/CUIT {} is not valid", cuil_cuit),
        Err(e) => println!("{}", e),
    }
}
