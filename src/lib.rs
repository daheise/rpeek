pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let source_file:parser::SourceText = parser::Parser::new(Path::new("Cargo.toml"));
    }
}
