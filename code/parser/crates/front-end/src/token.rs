/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use core::fmt;
use std::fmt::Write;

// 记录 Token 在源文件中的位置
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file_id: usize, // 源文件 id
    pub start: usize,   // 开始位置
    pub end: usize,     // 结束位置（不包括）
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    NewLine,            // 换行符号，包括 '\r\n', '\n'， '\r' 以及 ';'
    Identifier(String), // 标识符

    // 字面量
    Integer(i64),   // 123, 1_001, 0xab, 0b1001
    Float(f64),     // 3.14, 1.6e-23。考虑将指数（指数只支持整数）分离出来
    Imaginary(f64), // 3i, 9.9i。考虑将指数（指数只支持整数）分离出来

    Bit(usize, Vec<u8>),    // 4'b1010, 8'xff, 8'd10
    Boolean(bool),          // true, false
    Char(char),             // 'a', '\x41', '\u{6587}'
    GeneralString(String),  // "foo"
    TemplateString(String), // `foo`
    HashString(String),     // #foo
    Regexp(String),   // ~/foo/
    Attribute(String),      // #[test] annotation

    // 符号
    //
    // 符号名称参考
    // https://en.wikipedia.org/wiki/List_of_typographical_symbols_and_punctuation_marks
    //
    // 运算符优先级参考
    // https://en.wikipedia.org/wiki/Operators_in_C_and_C%2B%2B#Operator_precedence
    LeftBrace,  // {
    RightBrace, // }

    Assign,                // =
    Pipe,                  // |
    LogicOr,               // ||
    LogicAnd,              // &&
    Equal,                 // ==
    NotEqual,              // !=
    GreaterThan,           // >
    GreaterThanOrEqual,    // >=
    LessThan,              // <
    LessThanOrEqual,       // <=
    NamedOperator(String), // :name:
    Concat,                // ++
    Plus,                  // +         // std::operator::Add
    Minus,                 // -         // std::operator::Sub|Neg
    Asterisk,              // *         // std::operator::Mul
    Slash,                 // /         // std::operator::Div

    OptionalOr,  // ??
    OptionalAnd, // ->

    Combine, // &
    Cast,    // ^
    Unwrap,  // ?
    Dot,     // .

    LeftBracket,  // [
    RightBracket, // ]

    Exclamation, // !

    LeftParen,  // (
    RightParen, // )

    // 其他符号
    At,                // @
    Interval,          // ..
    IntervalInclusive, // ..=
    Ellipsis,          // ...
    Separator,         // ::
    Colon,             // :
    Comma,             // ,

    // 关键字
    Do,
    Join,

    Let,
    Fn,
    Sign,

    If,
    Then,
    Else,
    For,
    Next,
    Each,
    In,

    Branch,
    Match,
    Case,
    Default,
    Where,
    Only,
    // As,
    Into,
    Regular,
    Template,

    Function,
    Type,
    Which,
    Empty,
    Pattern,
    Limit,

    // Namespace,
    Use,
    Const,
    Enum,
    Struct,
    Union,
    Trait,
    Impl,
    Alias,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenDetail {
    pub location: Location,
    pub token: Token,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "file id: {}, start: {}, end: {}",
            self.file_id, self.start, self.end
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::NewLine => write!(f, "\n"),
            Token::Identifier(value) => write!(f, "{}", value),

            Token::Integer(value) => write!(f, "{}", value),
            Token::Float(value) => write!(f, "{}", value),
            Token::Imaginary(value) => write!(f, "{}i", value),
            Token::Bit(width, bytes) => {
                let mut hex = String::new();
                for byte in bytes {
                    write!(hex, "{:02x}", byte)?;
                }
                write!(f, "{}'x{}", width, hex)
            }
            Token::Boolean(value) => write!(f, "{}", value),
            Token::Char(value) => write!(f, "'{}'", value),
            Token::GeneralString(value) => write!(f, "\"{}\"", value),
            Token::TemplateString(value) => write!(f, "`{}`", value),
            Token::HashString(value) => write!(f, "#{}", value),
            Token::Regexp(value) => write!(f, "~/{}/", value),
            Token::Attribute(value) => write!(f, "#[{}]", value),

            Token::LeftBrace => write!(f, "{{"),  // {
            Token::RightBrace => write!(f, "}}"), // }

            Token::Assign => write!(f, "="),              // =
            Token::Pipe => write!(f, "|"),                // |
            Token::LogicOr => write!(f, "||"),            // ||
            Token::LogicAnd => write!(f, "&&"),           // &&
            Token::Equal => write!(f, "=="),              // ==
            Token::NotEqual => write!(f, "!="),           // !=
            Token::GreaterThan => write!(f, ">"),         // >
            Token::GreaterThanOrEqual => write!(f, ">="), // >=
            Token::LessThan => write!(f, "<"),            // <
            Token::LessThanOrEqual => write!(f, "<="),    // <=
            Token::NamedOperator(value) => write!(f, ":{}:", value), // :name:
            Token::Concat => write!(f, "++"),             // ++
            Token::Plus => write!(f, "+"),                // + Add
            Token::Minus => write!(f, "-"),               // - Subtract
            Token::Asterisk => write!(f, "*"),            // * Multiply
            Token::Slash => write!(f, "/"),               // / Divide

            Token::OptionalOr => write!(f, "??"),  // ??
            Token::OptionalAnd => write!(f, "->"), // ->

            Token::Combine => write!(f, "&"), // &
            Token::Cast => write!(f, "^"),    // ^
            Token::Unwrap => write!(f, "?"),  // ?
            Token::Dot => write!(f, "."),     // .

            Token::LeftBracket => write!(f, "["),  // [
            Token::RightBracket => write!(f, "]"), // ]

            Token::Exclamation => write!(f, "!"), // !

            Token::LeftParen => write!(f, "("),  // (
            Token::RightParen => write!(f, ")"), // )

            // 其他符号
            Token::At => write!(f, "@"),                  // ..
            Token::Interval => write!(f, ".."),           // ..
            Token::IntervalInclusive => write!(f, "..="), // ..=
            Token::Ellipsis => write!(f, "..."),          // ...
            Token::Separator => write!(f, "::"),          // ::
            Token::Colon => write!(f, ":"),               // :
            Token::Comma => write!(f, ","),               // ,

            // 关键字
            Token::Do => write!(f, "do"),
            Token::Join => write!(f, "join"),

            Token::Let => write!(f, "let"),
            Token::Fn => write!(f, "fn"),
            Token::Sign => write!(f, "sign"),

            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::For => write!(f, "for"),
            Token::Next => write!(f, "next"),
            Token::Each => write!(f, "each"),
            Token::In => write!(f, "in"),

            Token::Branch => write!(f, "branch"),
            Token::Match => write!(f, "match"),
            Token::Case => write!(f, "case"),
            Token::Default => write!(f, "default"),
            Token::Where => write!(f, "where"),
            Token::Only => write!(f, "only"),
            // Token::As => write!(f, "as"),
            Token::Into => write!(f, "into"),
            Token::Regular => write!(f, "regular"),
            Token::Template => write!(f, "template"),

            Token::Function => write!(f, "function"),
            Token::Type => write!(f, "type"),
            Token::Which => write!(f, "which"),
            Token::Empty => write!(f, "empty"),
            Token::Pattern => write!(f, "pattern"),
            Token::Limit => write!(f, "limit"),

            // Token::Namespace => write!(f, "namespace"),
            Token::Use => write!(f, "use"),
            Token::Const => write!(f, "const"),
            Token::Enum => write!(f, "enum"),
            Token::Struct => write!(f, "struct"),
            Token::Union => write!(f, "union"),
            Token::Trait => write!(f, "trait"),
            Token::Impl => write!(f, "impl"),
            Token::Alias => write!(f, "alias"),
        }
    }
}

impl fmt::Display for TokenDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", &self.location, &self.token)
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, Token, TokenDetail};

    #[test]
    fn test_location_display() {
        let ca1 = Location {
            file_id: 1,
            start: 2,
            end: 3,
        };

        assert_eq!(ca1.to_string(), "file id: 1, start: 2, end: 3");
    }

    #[test]
    fn test_location_eq() {
        let ca1 = Location {
            file_id: 1,
            start: 2,
            end: 3,
        };
        let ca2 = Location {
            file_id: 2,
            start: 2,
            end: 3,
        };
        let ca3 = Location {
            file_id: 1,
            start: 2,
            end: 3,
        };

        assert_ne!(ca1, ca2);
        assert_eq!(ca1, ca3);
    }

    #[test]
    fn test_token_display() {
        assert_eq!(Token::NewLine.to_string(), "\n");
        assert_eq!(Token::Identifier("foo".to_string()).to_string(), "foo");

        assert_eq!(Token::Integer(123).to_string(), "123");
        assert_eq!(Token::Float(6.626).to_string(), "6.626");
        assert_eq!(Token::Imaginary(0.618).to_string(), "0.618i");
        assert_eq!(Token::Bit(8, vec![0xab, 0x4]).to_string(), "8'xab04");
        assert_eq!(Token::Boolean(true).to_string(), "true");
        assert_eq!(Token::Char('a').to_string(), "'a'");
        assert_eq!(
            Token::GeneralString("foo".to_string()).to_string(),
            "\"foo\""
        );
        assert_eq!(
            Token::TemplateString("foo".to_string()).to_string(),
            "`foo`"
        );
        assert_eq!(Token::HashString("foo".to_string()).to_string(), "#foo");
        assert_eq!(Token::Regexp("foo".to_string()).to_string(), "~/foo/");
        assert_eq!(Token::Attribute("test".to_string()).to_string(), "#[test]");

        assert_eq!(Token::NamedOperator("foo".to_string()).to_string(), ":foo:");

        assert_eq!(Token::LeftBrace.to_string(), "{");
        assert_eq!(Token::RightBrace.to_string(), "}");

        assert_eq!(Token::Assign.to_string(), "=");
        assert_eq!(Token::Pipe.to_string(), "|");
        assert_eq!(Token::LogicOr.to_string(), "||");
        assert_eq!(Token::LogicAnd.to_string(), "&&");
        assert_eq!(Token::Equal.to_string(), "==");
        assert_eq!(Token::NotEqual.to_string(), "!=");
        assert_eq!(Token::GreaterThan.to_string(), ">");
        assert_eq!(Token::GreaterThanOrEqual.to_string(), ">=");
        assert_eq!(Token::LessThan.to_string(), "<");
        assert_eq!(Token::LessThanOrEqual.to_string(), "<=");
        assert_eq!(Token::Concat.to_string(), "++");
        assert_eq!(Token::Plus.to_string(), "+");
        assert_eq!(Token::Minus.to_string(), "-");
        assert_eq!(Token::Asterisk.to_string(), "*");
        assert_eq!(Token::Slash.to_string(), "/");

        assert_eq!(Token::OptionalOr.to_string(), "??");
        assert_eq!(Token::OptionalAnd.to_string(), "->");

        assert_eq!(Token::Combine.to_string(), "&");
        assert_eq!(Token::Cast.to_string(), "^");
        assert_eq!(Token::Unwrap.to_string(), "?");
        assert_eq!(Token::Dot.to_string(), ".");

        assert_eq!(Token::LeftBracket.to_string(), "[");
        assert_eq!(Token::RightBracket.to_string(), "]");

        assert_eq!(Token::Exclamation.to_string(), "!");
        assert_eq!(Token::LeftParen.to_string(), "(");
        assert_eq!(Token::RightParen.to_string(), ")");

        assert_eq!(Token::At.to_string(), "@");
        assert_eq!(Token::IntervalInclusive.to_string(), "..=");
        assert_eq!(Token::Interval.to_string(), "..");
        assert_eq!(Token::Ellipsis.to_string(), "...");
        assert_eq!(Token::Separator.to_string(), "::");
        assert_eq!(Token::Colon.to_string(), ":");
        assert_eq!(Token::Comma.to_string(), ",");

        assert_eq!(Token::Do.to_string(), "do");
        assert_eq!(Token::Join.to_string(), "join");

        assert_eq!(Token::Let.to_string(), "let");
        assert_eq!(Token::Fn.to_string(), "fn");
        assert_eq!(Token::Sign.to_string(), "sign");

        assert_eq!(Token::If.to_string(), "if");
        assert_eq!(Token::Then.to_string(), "then");
        assert_eq!(Token::Else.to_string(), "else");
        assert_eq!(Token::For.to_string(), "for");
        assert_eq!(Token::Next.to_string(), "next");
        assert_eq!(Token::Each.to_string(), "each");
        assert_eq!(Token::In.to_string(), "in");

        assert_eq!(Token::Branch.to_string(), "branch");
        assert_eq!(Token::Match.to_string(), "match");
        assert_eq!(Token::Case.to_string(), "case");
        assert_eq!(Token::Default.to_string(), "default");
        assert_eq!(Token::Where.to_string(), "where");
        assert_eq!(Token::Only.to_string(), "only");
        assert_eq!(Token::Into.to_string(), "into");
        assert_eq!(Token::Regular.to_string(), "regular");
        assert_eq!(Token::Template.to_string(), "template");

        assert_eq!(Token::Function.to_string(), "function");
        assert_eq!(Token::Type.to_string(), "type");
        assert_eq!(Token::Which.to_string(), "which");
        assert_eq!(Token::Empty.to_string(), "empty");
        assert_eq!(Token::Pattern.to_string(), "pattern");
        assert_eq!(Token::Limit.to_string(), "limit");

        // assert_eq!(Token::Namespace.to_string(), "namespace");
        assert_eq!(Token::Use.to_string(), "use");
        assert_eq!(Token::Const.to_string(), "const");
        assert_eq!(Token::Enum.to_string(), "enum");
        assert_eq!(Token::Struct.to_string(), "struct");
        assert_eq!(Token::Union.to_string(), "union");
        assert_eq!(Token::Trait.to_string(), "trait");
        assert_eq!(Token::Impl.to_string(), "impl");
        assert_eq!(Token::Alias.to_string(), "alias");
    }

    #[test]
    fn test_token_eq() {
        let tt1 = Token::Identifier("foo".to_string());
        let tt2 = Token::GeneralString("foo".to_string());
        let tt3 = Token::Identifier("foo".to_string());

        assert_ne!(tt1, tt2);
        assert_eq!(tt1, tt3);

        assert_ne!(Token::Let, Token::For);
        assert_eq!(Token::Let, Token::Let);
    }

    #[test]
    fn test_token_detail_display() {
        let tk1 = TokenDetail {
            location: Location {
                file_id: 1,
                start: 2,
                end: 3,
            },
            token: Token::Plus, // Add
        };

        let tk2 = TokenDetail {
            location: Location {
                file_id: 1,
                start: 2,
                end: 3,
            },
            token: Token::Minus, // Subtract
        };

        assert_eq!(tk1.to_string(), "[file id: 1, start: 2, end: 3] +");
        assert_ne!(tk1, tk2);
    }
}
