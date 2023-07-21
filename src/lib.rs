pub struct File {
    pub pid: u32,
    pub comm: String,
}

impl ToString for File {
    fn to_string(&self) -> String {
        format!("{} {}", self.pid, self.comm)
    }
}

pub struct Fist;

impl Fist {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> Vec<File> {
        procfs::process::all_processes()
            .unwrap()
            .map(|prc| {
                let prc = prc.unwrap();
                let stat = prc.stat().unwrap();
                File {
                    pid: stat.pid as u32,
                    comm: stat.comm,
                }
            })
            .collect()
    }
}
