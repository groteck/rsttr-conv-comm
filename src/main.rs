// This is a grammar module using rust_sitter to parse conventional commits
// Specs: https://www.conventionalcommits.org/en/v1.0.0/
//
// <type>[optional scope]: <description>
//
// [optional body]
//
// [optional footer(s)]
//
#[rust_sitter::grammar("commit")]
mod grammar {
    #[derive(Debug)]
    #[rust_sitter::language]
    pub struct Language {
        pub type_: Type,
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
        #[rust_sitter::leaf(pattern = r".+", transform = |v| v.to_string())]
        pub description: String,
        pub footer: Option<Footer>,
        pub body: Option<Body>,
    }

    #[derive(Debug)]
    pub struct Type {
        #[rust_sitter::leaf(pattern = r"feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert", transform = |v| v.to_string())]
        pub value: String,
        #[rust_sitter::leaf(pattern = r"\((.+)\)", transform = |v| v.to_string())]
        pub scope: Option<String>,
        #[rust_sitter::leaf(text = ":")]
        pub _separator: (),
    }

    #[derive(Debug)]
    pub struct Body {
        #[rust_sitter::leaf(pattern = r"\n|\n\n")]
        _prefix: (),
        #[rust_sitter::leaf(pattern = r".+", transform = |v| v.to_string())]
        pub value: Option<String>,
    }

    #[derive(Debug)]
    pub struct FooterLine {
        #[rust_sitter::leaf(pattern = r".+", transform = |v| v.to_string())]
        pub tag: String,
        #[rust_sitter::leaf(text = ":")]
        pub _separator: (),
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
        #[rust_sitter::leaf(pattern = r".+", transform = |v| v.to_string())]
        pub value: String,
    }

    #[derive(Debug)]
    pub struct Footer {
        #[rust_sitter::leaf(pattern = r"\n\n")]
        _prefix: (),
        #[rust_sitter::repeat(non_empty = true)]
        #[rust_sitter::delimited(
            #[rust_sitter::leaf(text = "\n")]
            ()
        )]
        pub lines: Vec<FooterLine>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_type_with_scope_and_description() {
        let input = "feat(scope): this is a commit decription";
        let tree = grammar::parse(input).unwrap();
        let type_ = tree.type_;
        assert_eq!(type_.value, "feat".to_string());
        assert_eq!(type_.scope, Some("(scope)".to_string()));
        assert_eq!(tree.description, "this is a commit decription".to_string());
    }

    #[test]
    fn parse_type_with_description() {
        let input = "feat: this is a commit decription";
        let tree = grammar::parse(input).unwrap();
        let type_ = tree.type_;
        assert_eq!(type_.value, "feat".to_string());
        assert_eq!(type_.scope, None);
        assert_eq!(tree.description, "this is a commit decription".to_string());
    }

    #[test]
    fn parse_body() {
        let input = r#"feat: this is a commit decription

This is a body"#;
        let tree = grammar::parse(input).unwrap();
        let type_ = tree.type_;
        let body = tree.body.unwrap();
        assert_eq!(type_.value, "feat".to_string());
        assert_eq!(type_.scope, None);
        assert_eq!(tree.description, "this is a commit decription".to_string());
        assert_eq!(body.value, Some("This is a body".to_string()));
    }

    #[test]
    fn parse_footer() {
        let input = r#"feat: this is a commit decription

BREAKING CHANGE: `extends` key in config file is now used for extending other config files"#;
        let tree = grammar::parse(input).unwrap();
        let type_ = tree.type_;
        let body = tree.body.unwrap();
        assert_eq!(type_.value, "feat".to_string());
        assert_eq!(type_.scope, None);
        assert_eq!(tree.description, "this is a commit decription".to_string());
        assert_eq!(body.value, None);
    }
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let tree = grammar::parse(&src).unwrap();

    println!("{tree:#?}");
}
