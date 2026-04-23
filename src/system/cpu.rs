pub struct CPU {
    pub core: usize,
    pub thread: usize,
    pub frequency: f64,
    pub usage: f64,
}

impl CPU {
    pub fn default() -> Self {
        Self {
            core: 4,        // Default to a quad-core CPU
            thread: 8,      // Default to 8 threads (with hyper-threading)
            frequency: 3.5, // Default to 3.5 GHz
            usage: 0.0,     // Default to 0% usage
        }
    }
}
