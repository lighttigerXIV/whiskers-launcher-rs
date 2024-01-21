use std::fs;
use bincode::deserialize;
use simple_kl_rs::api::extensions;
use simple_kl_rs::api::extensions::{Action, Context};
use simple_kl_rs::paths::get_extension_context_path;

fn main() {

    let action = extensions::Action::RunAction;
    let context = extensions::Context::new(action).search_text("banana");

    println!("Context Before: {:?}", context);

    let serialized_context = bincode::serialize(&context).unwrap();
    fs::write(get_extension_context_path().unwrap(), &serialized_context).unwrap();

    let deserialized_context: Context = deserialize(&fs::read(get_extension_context_path().unwrap()).unwrap()).unwrap();

    println!("Context: {:?}", deserialized_context);
}
