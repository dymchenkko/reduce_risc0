use risc0_zkvm_host::Receipt;
use risc0_zkvm_serde::from_slice;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("receipt")?;
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    let rec: Receipt = serde_cbor::from_slice(&buffer).unwrap();
    let result: bool = from_slice(&rec.get_journal_vec().unwrap()).unwrap();
    println!("I know the factors of {:?}, and I can prove it!", result);
    Ok(())
}
