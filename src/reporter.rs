pub struct Reporter {
    pub errors: usize,
    pub warnings: usize,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            errors: 0,
            warnings: 0,
        }
    }

    pub fn error(&mut self, msg: impl AsRef<str>) {
        println!("  ERROR: {}", msg.as_ref());
        self.errors += 1;
    }

    pub fn warn(&mut self, msg: impl AsRef<str>) {
        println!("  WARNING: {}", msg.as_ref());
        self.warnings += 1;
    }
}
