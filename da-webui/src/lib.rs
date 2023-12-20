pub const CORE: &str = include_str!("lib.js");

pub fn create_js(custom: &str) -> String {
    format!("{CORE}\n{custom}\n")
}