use crate::{error::make_error, sources::span::CodeArea};

use super::tokens::Token;

make_error! {
    @kind: Error;

    ParserError {

        @title: format!("Expected {}, found `{}`", expected, found.name());
        @msgs: [
            area => "Expected {}": expected;
        ];
        Expected {
            expected: String,
            found: Token,
            area: CodeArea,
        }

    }
}
