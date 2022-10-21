#! [allow(dead_code)]

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}


fn main() {
    let _command = Command::builder()
        .executable("jhdsjkhds".to_string())
        .current_dir("jhdsjkhds".to_string())
        .env(vec![])
        .args(vec![])

        .build()
        .unwrap();

    println!("{}", _command.executable);
}
