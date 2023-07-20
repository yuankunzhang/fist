pub struct Process {
    pub pid: i32,
}

pub struct Fist;

impl Fist {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        println!("Hello from Fu");
    }
}
