use crate::ast;

pub fn parse(input: &str) -> ast::AST<'_> {
    let ast = ast::AST {
        statements: Vec::new(),
    };

    ast
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ast = parse(
            r#"
defn hello (i) :
  2 + 3

5 + 10
            "#,
        );
    }
}
