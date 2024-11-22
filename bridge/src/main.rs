use std::path::{Path, PathBuf};

use anyhow::bail;
use clap::{Arg, Parser};
use wasmer::{imports, Instance, Module, Store, Value};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandLineArguments {
    /// Name of the person to greet
    #[arg(short, long)]
    plugin: String,
}

fn main() -> anyhow::Result<()> {
    let cmd_args = CommandLineArguments::parse();

    let mut store = Store::default();
    let module = Module::from_file(&store, PathBuf::from(&cmd_args.plugin))?;

    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let entry = instance.exports.get_global("clap_entry")?;

    let Some(entry_value) = entry.get(&mut store).externref() else {
        bail!("couldn't get clap_entry value");
    };


    // let add_one = instance.exports.get_function("add_one")?;
    // let result = add_one.call(&mut store, &[Value::I32(42)])?;
    // assert_eq!(result[0], Value::I32(43));

    Ok(())
}
