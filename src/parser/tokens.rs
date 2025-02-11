use crate::sources::span::Span;
use logos::Logos;

macro_rules! tokens {
    (
        $(
            $( @token($toklit:literal) )?
            $( @regex($reglit:literal) )?
            $( @name($namelit:literal) )?
            $name:ident,
        )*
    ) => {

        pub trait TokenKind: std::fmt::Debug + PartialEq + Eq + Clone + Copy {
            fn name(self) -> &'static str;
        }

        #[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
        #[logos(skip r"[ \t\r\n\f]+")]
        enum LogosToken {
            $(
                $( #[token($toklit)] )?
                $( #[regex($reglit)] )?
                $name,
            )*
        }

        paste::paste! {

            $(
                #[derive(Debug, PartialEq, Eq, Clone, Copy)]
                pub struct [< Token $name >] {
                    span: Span,
                }

                impl TokenKind for [< Token $name >] {
                    fn name(self) -> &'static str {
                        $( $toklit )? $( $namelit )?
                    }
                }

                impl super::Parse for [< Token $name >] {
                    fn parse(parser: &mut super::Parser) -> Self {
                        parser.
                    }
                }
            )*

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            enum Token {
                $(
                    $name([< Token $name >]),
                )*
            }

            impl LogosToken {
                fn to_tok(self, span: Span) -> Token {
                    match self {
                        $(
                            Self::$name => Token::$name([< Token $name >] { span }),
                        )*
                    }
                }
            }
        }




    };
}

tokens! {
    @regex("[a-zA-Z_][a-zA-Z_0-9]*") @name("identifier")
    Ident,
    @regex("[0-9]+") @name("integer literal")
    Integer,
    @regex("[0-9]+\\.([0-9]*)?") @name("float literal")
    Float,
    @regex(r##""(?:[^"]|\\")*""##) @name("string literal")
    String,

    @token("+")
    Plus,
    @token("-")
    Minus,
    @token("*")
    Asterisk,
    @token("/")
    Div,
    @token("%")
    Mod,

    @token("=")
    Assign,
    @token("+=")
    PlusAssign,
    @token("-=")
    MinusAssign,
    @token("*=")
    MultAssign,
    @token("/=")
    DivAssign,
    @token("%=")
    ModAssign,

    @token("==")
    Eq,
    @token("!=")
    NEq,
    @token("<")
    Lt,
    @token(">")
    Gt,
    @token("<=")
    LtE,
    @token(">=")
    GtE,

    @token("(")
    OpenParen,
    @token(")")
    CloseParen,
    @token("[")
    OpenSquare,
    @token("]")
    CloseSquare,
    @token("{")
    OpenCurly,
    @token("}")
    CloseCurly,

    @token(",")
    Comma,
    @token(";")
    Semicolon,
    @token(":")
    Colon,

    @token("=>")
    FatArrow,

    @token("true")
    True,
    @token("false")
    False,

    @token("let")
    Let,
    @token("if")
    If,
    @token("else")
    Else,
    @token("while")
    While,
    @token("for")
    For,

    @token("dbg")
    Dbg,

    @name("unknown")
    Unknown,
    @name("end of file")
    Eof,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, LogosToken>,
}
impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            inner: LogosToken::lexer(src),
        }
    }
    pub fn span(&self) -> Span {
        let span = self.inner.span();
        Span::new(span.start, span.end)
    }
    pub fn slice(&self) -> &str {
        self.inner.slice()
    }
    pub fn next(&mut self) -> Token {
        self.inner
            .next()
            .map(|v| v.unwrap_or(LogosToken::Unknown))
            .unwrap_or(LogosToken::Eof)
            .to_tok(self.span())
    }
}
