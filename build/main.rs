pub struct Build;

#[allow(unused)]
impl Build {
    pub fn new() -> Self {
        Self {}
    }
    pub fn warn(&self, msg: &str) {
        println!("cargo::warning={}", msg);
    }
    pub fn error(&self, msg: &str) {
        println!("cargo::error={}", msg);
    }
}

fn main() {
    let _build = Build::new();
}
