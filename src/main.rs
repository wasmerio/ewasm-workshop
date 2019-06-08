extern crate wasmer_runtime;
extern crate rand;

mod utils;

use std::str;

use wasmer_runtime::{
    imports,
    instantiate,
    error,
    func,
    Ctx,
    compile,
    compile_with
};
use wasmer_middleware_common::metering;

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] = include_bytes!("../smartcontract/target/wasm32-unknown-unknown/release/wasm_smart_contract.wasm");

fn main() -> error::Result<()> {
    let import_object = imports! {
        "env" => {
            "random_number" => func!(utils::random_number),
        },
    };

    let module = compile_with(&WASM, &utils::get_metered_compiler(6, true)).unwrap();

    // Compile our webassembly into an `Instance`.
    let instance = module.instantiate(&import_object)?;

    // Multiply by two
    let result = instance.call("multiply_by_two", &[32.into()])?;

    println!("Multiply by two: {}", result[0].to_u64());

    // Multiply by random
    let result = instance.call("multiply_by_random", &[0.into()])?;

    let used = metering::get_points_used(&instance);

    println!("Multiply by random: {}. Points used: {}", result[0].to_u64(), used);

    Ok(())
}
