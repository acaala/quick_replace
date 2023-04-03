pub struct Config {
    pub from: String,
    pub to: String,
    pub file_path: String,
    pub replace: bool,
    pub compress: bool,
}

impl Config {
    pub fn build(from: String, to: String, file_path: String, replace: bool, compress: bool) -> Config {
        Config { from, to, file_path, replace, compress }
    }
}
