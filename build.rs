use vergen::{Config, vergen};

fn main() {
  // Generate the default 'cargo:' instruction output
  vergen(Config::default()).expect("Wtf wtf wtf wtf");
}