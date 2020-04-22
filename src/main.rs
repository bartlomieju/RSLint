use rslint::linter::Linter;

fn main() {
  env_logger::init();
  Linter::new(String::from("tests")).run()
    .expect("Failed to run linter");
}