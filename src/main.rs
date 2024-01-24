use std::fs;
use simple_kl_rs::actions::Action;
use simple_kl_rs::api::extensions::{get_extension_results, write_extension_results};
use simple_kl_rs::paths::get_extension_results_path;
use simple_kl_rs::results;
use simple_kl_rs::results::SimpleKLResult;

fn main() {

    let first_text = results::Text::new("bulbasaur", Action::Nothing);
    let second_text = results::Text::new("charmander", Action::Nothing);
    let third_text = results::Text::new("squirtle", Action::Nothing);

    let results = vec![
        SimpleKLResult::Text(first_text),
        SimpleKLResult::Text(second_text),
        SimpleKLResult::Text(third_text),
    ];

    write_extension_results(results);

    let file_results = get_extension_results().unwrap();
    println!("File results -> {:?}", file_results);
}
