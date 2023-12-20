pub const core: &str = include_str!("lib.js");

pub fn create_js(custom: &str) -> String {
    format!("{core}\n{custom}\n")
}