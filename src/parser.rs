/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use crate::{
    ast::{
        AnonymousFunction, AnonymousParameter, Argument, BinaryExpression, Bit, BlockExpression,
        Boolean, Char, Complex, ConstructorExpression, DataType, Ellipsis, Expression, Float,
        FunctionCallExpression, GeneralString, HashString, Identifier, Integer, Interval,
        JoinExpression, LetExpression, List, Literal, Map, MapEntry, MemberExpression, MemberIndex,
        MemberProperty, NamedOperator, Node, PrefixIdentifier, Program, Range, Sign, SignParameter,
        Statement, Tuple, UnaryExpression, WhichEntry, WhichEntryLimit, WhichEntryType,
    },
    error::Error,
    token::{Token, TokenDetail},
};

pub fn parse(source_token_details: &[TokenDetail]) -> Result<Node, Error> {
    let program = parse_program(source_token_details)?;
    Ok(Node::Program(program))
}

// Program
//  : StatementList
//  ;
//
// StatementList
//  : Statement
//  | StatementList NEW_LINE Statement
//  ;
fn parse_program(source_token_details: &[TokenDetail]) -> Result<Program, Error> {
    let mut token_details = source_token_details;
    let mut statements = Vec::<Statement>::new();

    loop {
        // 消除前导的空行
        let post_new_lines = skip_new_lines(token_details);

        if post_new_lines.first() == None {
            break;
        }

        let (statement, post_statement) = parse_statement(post_new_lines)?;
        statements.push(statement);

        // 解析剩余的 token
        // 直到解析完所有 token 为止
        token_details = post_statement;
    }

    Ok(Program {
        body: statements,
        range: new_range(),
    })
}

// Statement
//  : FunctionDeclaration
//  | EmptyFunctionDeclaration
//  | PatternFunctionDeclaration
//
//  | NamespaceStatement
//  | UseStatement
//  | ConstDeclaration
//
//  | MemberStructDeclaration
//  | TupleStructDeclaration
//  | EmptyStructDeclaration
//
//  | UnionDeclaration
//  | TraitDeclaration
//  | ImplStatement
//  | AliasStatement
//  | Expression
//  ;
fn parse_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    let first = &source_token_details[0];
    match first.token {
        Token::Function => parse_function_declaration(source_token_details),
        Token::Empty => parse_empty_function_declaration(source_token_details),
        Token::Pattern => parse_pattern_function_declaration(source_token_details),
        Token::Namespace => parse_namespace_statement(source_token_details),
        Token::Use => parse_use_statement(source_token_details),
        Token::Const => parse_const_statement(source_token_details),
        Token::Struct => parse_struct(source_token_details),
        Token::Union => parse_union(source_token_details),
        Token::Trait => parse_trait_declaration(source_token_details),
        Token::Impl => parse_impl_statement(source_token_details),
        Token::Alias => parse_alias_statement(source_token_details),
        _ => {
            // 表达式语句
            parse_expression_statement(source_token_details)
        }
    }
}

fn parse_function_declaration(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_empty_function_declaration(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_pattern_function_declaration(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_namespace_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_use_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_const_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_struct(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_union(source_token_details: &[TokenDetail]) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_trait_declaration(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_impl_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_alias_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

// ExpressionStatement
//  : Expression NEW_LINE
//  | Expression EOF
//  ;
fn parse_expression_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    let (expression, rest) = parse_expression(source_token_details)?;

    // statement 以 Token::NewLine 或者 EOF 结束，消耗这个换行符（如果存在的话）
    consume_new_line_or_end_of_file(rest)
        .map(|post_rest| (Statement::Expression(expression), post_rest))
}

// Expression
//  : BlockExpression
//  | JoinExpression
//  | LetExpression
//
//  | IfExpression
//  | ForExpression
//  | NextExpression
//  | EachExpression
//  | BranchExpression
//  | MatchExpression
//
//  | BinaryExpression
//  | UnaryExpression
//  | FunctionCallExpression
//  | MemberExpression
//  | SliceExpression
//  | ConstructorExpression
//
//  | AnonymousFunction
//  | SignExpression
//  | Identifier
//  | PrefixIdentifier
//  | Tuple
//  | List
//  | Map
//  | Literal
//  ;

fn parse_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    if let Some(first) = source_token_details.first() {
        match first.token {
            Token::Do => parse_do_expression(source_token_details),
            Token::Join => parse_join_expression(source_token_details),
            Token::Let => parse_let_expression(source_token_details),
            Token::If => parse_if_expression(source_token_details),
            Token::For => parse_for_expression(source_token_details),
            Token::Next => parse_next_expression(source_token_details),
            Token::Each => parse_each_expression(source_token_details),
            Token::Branch => parse_branch_expression(source_token_details),
            Token::Match => parse_match_expression(source_token_details),
            _ => {
                // 二元运算表达式的开始
                parse_pipe_expression(source_token_details)
            }
        }
    } else {
        Err(Error::ParserError("expected expression".to_string()))
    }
}

// BlockExpression
//  : 'do' BlockExpression
//  ;
fn parse_do_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 do 表达式 `do {...}`，do 表达式是一个显式表达式块

    // 消除 do
    let post_consume_token_do = consume_token(&Token::Do, source_token_details)?;

    // 消除换行符
    // do 关键字后面允许换行
    let post_consume_new_lines = skip_new_lines(post_consume_token_do);

    let (expressions, post_expression_block) =
        continue_parse_expression_block(post_consume_new_lines)?;

    Ok((
        Expression::BlockExpression(BlockExpression {
            is_explicit: true,
            body: expressions,
            range: new_range(),
        }),
        post_expression_block,
    ))
}

// BlockExpression
//  : '{' ExpressionList '}'
//  ;
//
// ExpressionList
//  : Expression
//  | ExpressionList NEW_LINE Expression
//  ;
fn continue_parse_expression_block(
    source_token_details: &[TokenDetail],
) -> Result<(Vec<Expression>, &[TokenDetail]), Error> {
    // 解析表达式块 `{...}`（也叫 `隠式 Do 表达式`）
    // 注意表达式块仅存在某些关键字后面，比如 `join`、`do` 等，而不能单独存在，
    // 当一对花括号单独存在时，会被解析为 Map。
    let mut token_details = source_token_details;

    token_details = consume_token(&Token::LeftBrace, token_details)?;

    let mut expressions: Vec<Expression> = vec![];

    loop {
        // 左花括号 '{' 后面允许换行
        // 表达式之间也是以换行分隔
        let post_consume_new_lines = skip_new_lines(token_details);

        // 解析表达式
        let (expression, post_expression) = parse_expression(post_consume_new_lines)?;
        expressions.push(expression);

        token_details = post_expression;

        if is_token_ignore_new_lines(&Token::RightBrace, post_expression) {
            break;
        }
    }

    // 消除空行
    let post_consume_new_lines = skip_new_lines(token_details);
    let post_consume_token_right_brace = consume_token(&Token::RightBrace, post_consume_new_lines)?;

    Ok((expressions, post_consume_token_right_brace))
}

fn continue_parse_expression_block_or_single_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 `{...}` 或者 `...`
    // 在诸如 `if`、`then`、`else` 等关键字后面，即可以是单独一个表达式，
    // 也可以是一个表达式块。
    //
    // 解析优先级：
    // 1. 如果存在 `{...}`，则解析为隠式表达式块
    // 2. 否则解析为普通的表达式

    match source_token_details.first() {
        Some(first) => match first.token {
            Token::LeftBrace => {
                let (expressions, post_expression_block) =
                    continue_parse_expression_block(source_token_details)?;

                Ok((
                    Expression::BlockExpression(BlockExpression {
                        is_explicit: false,
                        body: expressions,
                        range: new_range(),
                    }),
                    post_expression_block,
                ))
            }
            _ => parse_expression(source_token_details),
        },
        None => Err(Error::ParserError(
            "expected an expression or an expression block".to_string(),
        )),
    }
}

fn parse_join_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 join 表达式
    // join {...}

    // 消除 join
    let post_consume_token_join = consume_token(&Token::Join, source_token_details)?;

    // 消除换行符
    // join 关键字后面允许换行
    let post_consume_new_lines = skip_new_lines(post_consume_token_join);

    let (expressions, post_expression_block) =
        continue_parse_expression_block(post_consume_new_lines)?;

    Ok((
        Expression::JoinExpression(JoinExpression {
            body: expressions,
            range: new_range(),
        }),
        post_expression_block,
    ))
}

fn parse_let_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 赋值表达式的左手边只允许如下几种表达式：
    // - Identifier
    // - Tuple
    // - List
    // - Map
    // - 数字、字符串等字面量
    //
    // let data_type left = right
    // let left = right // 数据类型可以省略
    // ~~~
    //   ^--- 当前所处的位置

    let mut token_details = source_token_details;

    // 消除关键字 `let`
    token_details = consume_token(&Token::Let, token_details)?;
    // 消除关键字 `let` 后面的空行
    token_details = skip_new_lines(token_details);

    // 解析左手边的数据类型或者值
    let (lhs_part_one, post_lhs_part_one) = parse_primary_expression(token_details)?;

    let (lhs_data_type, lhs_object) = if is_token(&Token::Assign, post_lhs_part_one) {
        // 当前表达式没有数据类型
        token_details = post_lhs_part_one;
        (None, lhs_part_one)
    } else {
        // 当前表达式有数据类型

        // 解析左手边值
        let (lhs_object, post_lhs_object) = parse_primary_expression(post_lhs_part_one)?;
        token_details = post_lhs_object;
        (Some(lhs_part_one), lhs_object)
    };

    // 消除赋值符号 `=`
    token_details = consume_token(&Token::Assign, token_details)?;

    // 消除赋值符号 `=` 后面的空行
    token_details = skip_new_lines(token_details);

    // 解析右手边值
    let (rhs_object, post_rhs_object) = parse_expression(token_details)?;

    let data_type = match lhs_data_type {
        Some(e) => {
            let d = convert_expression_to_data_type(e)?;
            Some(d)
        }
        None => None,
    };

    let exp = Expression::LetExpression(LetExpression {
        data_type: data_type,
        object: Box::new(lhs_object),
        value: Box::new(rhs_object),
        range: new_range(),
    });

    Ok((exp, post_rhs_object))
}

fn parse_if_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // if ... then ...
    // if ... then ... else ...
    // ~~
    //  ^--- 当前所处的位置

    let mut token_details = source_token_details;

    // 消除关键字 `if`
    token_details = consume_token(&Token::If, token_details)?;
    // 消除关键字 `if` 后面的空行
    token_details = skip_new_lines(token_details);

    let (condition, post_condition) = continue_parse_expression_block_or_single_expression(token_details)?;

    // 消除关键字 `then` (包括前缀空行)
    token_details = skip_new_lines_and_consume_token(&Token::Then, post_condition)?;
    // 消除关键字 `then` 后面的空行
    token_details = skip_new_lines(token_details);

    let (consequent, post_consequent) = continue_parse_expression_block_or_single_expression(token_details)?;

    // 检查是否存在 `else` 子表达式
    let alternate = if is_token_ignore_new_lines(&Token::Else, post_consequent) {
        // 消除关键字 `else` (包括前缀空行)
        token_details = skip_new_lines_and_consume_token(&Token::Then, post_consequent)?;
        // 消除关键字 `else` 后面的空行
        token_details = skip_new_lines(token_details);

        let (alternate, post_alternate) = continue_parse_expression_block_or_single_expression(token_details)?;

        token_details = post_alternate;
        Some(alternate)
    }else {
        token_details = post_consequent;
        None
    };

    todo!()
}

fn continue_parse_where_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // where ...
    // ~~~~~
    //     |-- 当前位置

    // 消除 `where` 关键字
    let post_type_token = consume_token(&Token::Where, source_token_details)?;
    // 消除空行
    let post_new_lines = skip_new_lines(post_type_token);

    continue_parse_expression_block_or_single_expression(post_new_lines)
}

fn parse_for_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // for let ... = ... {... next}
    todo!()
}

fn parse_next_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // for let ... = ... {... next}
    todo!()
}

fn parse_each_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // each let ... in ... {...}
    todo!()
}

fn parse_branch_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // branch {...}
    todo!()
}

fn parse_match_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // match ... {...}
    todo!()
}

fn continue_parse_generic_names(
    source_token_details: &[TokenDetail],
) -> Result<(Vec<DataType>, &[TokenDetail]), Error> {
    // <A>
    // <A, B, C>
    // <A, B<C>>
    // ^
    // |--- 当前所处的位置

    let mut token_details = source_token_details;

    let mut generics: Vec<DataType> = vec![];
    let mut is_expected_end = false; // 标记当前是否处于一心寻找结束符的状态

    // 消除符号 `<`
    token_details = consume_token(&Token::LessThan, token_details)?;
    // 消除符号 `<` 后面的空行
    token_details = skip_new_lines(token_details);

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if first.token == Token::GreaterThan {
                    // 找到了结束符号 `>`，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前的状态是一心寻找结束符号 `>`
                        return Err(Error::ParserError(
                            "expected the right angle bracket symbol \">\"".to_string(),
                        ));
                    } else {
                        // 寻找泛型的 `数据类型`
                        let (data_type_expression, post_primary_expression) =
                            parse_primary_expression(token_details)?;
                        let data_type = convert_expression_to_data_type(data_type_expression)?;

                        generics.push(data_type);

                        let post_comma = if is_token(&Token::Comma, post_primary_expression) {
                            consume_token(&Token::Comma, post_primary_expression)?
                        } else {
                            // 设置标记，表示如果项目后面没有逗号，则表示当前已经是最后一项
                            // 后面只能允许列表结束
                            is_expected_end = true;
                            post_primary_expression
                        };

                        // 消除符号 `,` 后面的空行
                        let post_new_lines = skip_new_lines(post_comma);
                        post_new_lines
                    }
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected the right angle bracket symbol \">\"".to_string(),
                ))
            }
        }
    }

    // 消除符号 `>`
    token_details = consume_token(&Token::GreaterThan, token_details)?;

    Ok((generics, token_details))
}

fn continue_parse_type_expression(
    source_token_details: &[TokenDetail],
) -> Result<(DataType, &[TokenDetail]), Error> {
    // type ...
    // ~~~~
    //    |-- 当前位置

    // 消除 `type` 关键字
    let post_type_token = consume_token(&Token::Type, source_token_details)?;
    // 消除空行
    let post_new_lines = skip_new_lines(post_type_token);

    let (data_type_expression, post_data_type_expression) = parse_expression(post_new_lines)?;
    let data_type = convert_expression_to_data_type(data_type_expression)?;
    Ok((data_type, post_data_type_expression))
}

fn continue_parse_which_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Vec<WhichEntry>, &[TokenDetail]), Error> {
    // which ...
    // which {...}
    // ~~~~~
    //     |-- 当前位置
    //
    // which T: std::Int                   // 单独一个数据类型
    // which T: limit std::Int + Display   // limit 有可能多个数据类型，多个数据类型之间使用加号拼接
    // which {                             // 多行格式
    //   T: Int,                           // 末尾的逗号可选
    //   E: limit Display + Eq             //
    //   U: String
    // }
    let mut token_details = source_token_details;
    let mut entries: Vec<WhichEntry> = vec![];

    let mut is_expected_end = false; // 标记当前是否处于一心寻找结束符的状态

    // 消除关键字 `which`
    token_details = consume_token(&Token::Which, token_details)?;
    // 消除空行，关键字后面允许空行
    token_details = skip_new_lines(token_details);

    let post_which_expression = match token_details.first() {
        Some(maybe_left_brace) => {
            if maybe_left_brace.token == Token::LeftBrace {
                // 解析 WhichEntry 表达式块

                // 消除 `{`
                token_details = consume_token(&Token::LeftBrace, token_details)?;
                // 消除空行
                token_details = skip_new_lines(token_details);

                loop {
                    token_details = match token_details.first() {
                        Some(maybe_right_brace) => {
                            if maybe_right_brace.token == Token::RightBrace {
                                // 找到结束符号 `}`，退出循环
                                break;
                            } else {
                                if is_expected_end {
                                    // 当前的状态是一心寻找结束符号 `}`
                                    return Err(Error::ParserError(
                                        "expected the right brace symbol \"}\"".to_string(),
                                    ));
                                } else {
                                    let (entry, post_entry) =
                                        continue_parse_which_entry(token_details)?;

                                    entries.push(entry);

                                    // 如果接下来是：
                                    // - 逗号
                                    // - 逗号+空行
                                    // - 空行
                                    //
                                    // 表明还有下一项，否则表示后面没有更多项目

                                    let post_consume_comma = match post_entry.split_first() {
                                        Some((first, rest)) if first.token == Token::Comma => {
                                            // 消除逗号
                                            rest
                                        }
                                        Some((first, _)) if first.token == Token::NewLine => {
                                            // 等接下来的代码来统一来消除空行
                                            post_entry
                                        }
                                        _ => {
                                            // 没有下一项了，标记映射表的已经到达末尾
                                            is_expected_end = true;
                                            post_entry
                                        }
                                    };

                                    // 消除空行
                                    let post_consume_new_lines = skip_new_lines(post_consume_comma);
                                    post_consume_new_lines
                                }
                            }
                        }
                        None => {
                            return Err(Error::ParserError(
                                "expected the right brace symbol \"}\"".to_string(),
                            ));
                        }
                    }
                }

                // 消除 `}`
                consume_token(&Token::RightBrace, token_details)?
            } else {
                // 解析单独一行的 WhichEntry
                let (entry, post_entry) = continue_parse_which_entry(token_details)?;
                entries.push(entry);

                post_entry
            }
        }
        None => {
            return Err(Error::ParserError(
                "expected \"which\" expression".to_string(),
            ));
        }
    };

    Ok((entries, post_which_expression))
}

fn continue_parse_which_entry(
    source_token_details: &[TokenDetail],
) -> Result<(WhichEntry, &[TokenDetail]), Error> {
    // 解析单一行 WhichEntry
    //
    // T: std::Int                  // 单独一个数据类型
    // T: limit std::Int + Display  // 多个数据类型
    // ^
    // |--- 当前所处的位置

    if let Some((
        TokenDetail {
            token: Token::Identifier(name),
            ..
        },
        post_name,
    )) = source_token_details.split_first()
    {
        // 消除符号 `:`
        let post_colon = consume_token(&Token::Colon, post_name)?;
        // 消除 `:` 之后的空行
        let post_new_lines_after_colon = skip_new_lines(post_colon);

        match post_new_lines_after_colon.split_first() {
            Some((maybe_token_limit, post_limit)) => {
                if maybe_token_limit.token == Token::Limit {
                    // 当前是泛型约束 `limit`

                    // 消除 `limit` 之后的空行
                    let post_new_lines_after_limit = skip_new_lines(post_limit);

                    let (data_types, post_data_type_list) =
                        continue_parse_which_entry_data_type_list(post_new_lines_after_limit)?;

                    let entry = WhichEntry::Limit(WhichEntryLimit {
                        name: name.clone(),
                        data_types: data_types,
                        range: new_range(),
                    });

                    Ok((entry, post_data_type_list))
                } else {
                    // 当前是单一数据类型说明
                    let (data_type_expression, post_data_type_expression) =
                        parse_expression(post_new_lines_after_colon)?;
                    let data_type = convert_expression_to_data_type(data_type_expression)?;

                    let entry = WhichEntry::Type(WhichEntryType {
                        name: name.clone(),
                        data_type: data_type,
                        range: new_range(),
                    });

                    Ok((entry, post_data_type_expression))
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected which expression entry value".to_string(),
                ));
            }
        }
    } else {
        return Err(Error::ParserError(
            "invalid name of which expression entry".to_string(),
        ));
    }
}

fn continue_parse_which_entry_data_type_list(
    source_token_details: &[TokenDetail],
) -> Result<(Vec<DataType>, &[TokenDetail]), Error> {
    // Display + Debug + Eq
    // ^
    // |--- 当前处在这个位置

    let mut token_details = source_token_details;

    let mut data_types: Vec<DataType> = vec![];

    loop {
        let (data_type_expression, post_data_type_expression) =
            parse_primary_expression(token_details)?;
        let data_type = convert_expression_to_data_type(data_type_expression)?;
        data_types.push(data_type);

        // let post_plus =
        if is_token(&Token::Plus, post_data_type_expression) {
            let post_plus = consume_token(&Token::Plus, post_data_type_expression)?;

            // 消除符号 `+` 后面的空行
            let post_new_lines = skip_new_lines(post_plus);
            token_details = post_new_lines;
        } else {
            token_details = post_data_type_expression;
            break;
        }
    }

    Ok((data_types, token_details))
}

// 解析 `从左向右` 结合的二元运算的通用函数
//
// BinaryExpression
//  : NextExpression
//  | BinaryExpression OPERATOR NextExpression
//  ;
fn parse_binary_expression<'a>(
    operator_tokens: &[Token],
    next_parse_function: fn(&[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error>,
    source_token_details: &'a [TokenDetail],
) -> Result<(Expression, &'a [TokenDetail]), Error> {
    let mut token_details = source_token_details;

    let (mut left, post_left_expression) = next_parse_function(token_details)?;
    token_details = post_left_expression;

    loop {
        let next_token = match token_details.first() {
            Some(first) => &first.token,
            None => {
                break;
            }
        };

        let index = match operator_tokens.iter().position(|t| t == next_token) {
            Some(i) => i,
            None => {
                break;
            }
        };

        let operator_token = &operator_tokens[index];

        // 消除操作符
        let post_consume_token_operator = consume_token(operator_token, token_details)?;

        // 二元运算符后面允许换行
        let post_consume_new_lines = skip_new_lines(post_consume_token_operator);

        let (right, post_right_expression) = next_parse_function(post_consume_new_lines)?;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
        token_details = post_right_expression;
    }

    Ok((left, token_details))
}

// 解析 `从右向左` 结合的二元运算的通用函数
//
// BinaryExpression
//  : NextExpression
//  | NextExpression OPERATOR Expression
//  ;
fn parse_right_2_left_binary_expression<'a>(
    operator_token: &Token,
    next_parse_function: fn(&[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error>,
    source_token_details: &'a [TokenDetail],
) -> Result<(Expression, &'a [TokenDetail]), Error> {
    let mut token_details = source_token_details;

    let (mut left, post_left_expression) = next_parse_function(token_details)?;
    token_details = post_left_expression;

    if is_token(operator_token, token_details) {
        // 消除操作符
        let post_consume_token_operator = consume_token(operator_token, token_details)?;

        // 二元运算符后面允许换行
        let pose_consume_new_lines = skip_new_lines(post_consume_token_operator);

        let (right, post_right_expression) = parse_expression(pose_consume_new_lines)?;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
        token_details = post_right_expression;
    }

    Ok((left, token_details))
}

fn parse_pipe_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left | right
    parse_binary_expression(
        &vec![Token::Pipe],
        parse_logic_or_expression,
        source_token_details,
    )
}

fn parse_logic_or_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left || right
    parse_binary_expression(
        &vec![Token::LogicOr],
        parse_logic_and_expression,
        source_token_details,
    )
}

fn parse_logic_and_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left && right
    parse_binary_expression(
        &vec![Token::LogicAnd],
        parse_equality_expression,
        source_token_details,
    )
}

fn parse_equality_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left == right, left != right
    parse_binary_expression(
        &vec![Token::Equal, Token::NotEqual],
        parse_relational_expression,
        source_token_details,
    )
}

fn parse_relational_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left > right, left >= right, left < right, left <= right
    parse_binary_expression(
        &vec![
            Token::GreaterThan,
            Token::GreaterThanOrEqual,
            Token::LessThan,
            Token::LessThanOrEqual,
        ],
        parse_named_operator_expression,
        source_token_details,
    )
}

fn parse_named_operator_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left :bitOr: right
    //
    // 注：
    // 命名操作符无法使用通用的二元运算解析函数 parse_binary_expression
    let mut token_details = source_token_details;

    let (mut left, post_left_expression) = parse_concat_expression(token_details)?;
    token_details = post_left_expression;

    if let Some(TokenDetail {
        token: named_operator_token @ Token::NamedOperator(_),
        ..
    }) = token_details.first()
    {
        // 消除操作符
        let post_consume_token_operator = consume_token(named_operator_token, token_details)?;

        // 二元运算符后面允许换行
        let pose_consume_new_lines = skip_new_lines(post_consume_token_operator);

        let (right, post_right_expression) = parse_concat_expression(pose_consume_new_lines)?;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: named_operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
        token_details = post_right_expression;
    }

    Ok((left, token_details))
}

fn parse_concat_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left ++ right
    parse_binary_expression(
        &vec![Token::Concat],
        parse_additive_expression,
        source_token_details,
    )
}

fn parse_additive_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left + right, left - right
    parse_binary_expression(
        &vec![Token::Plus, Token::Minus],
        parse_multiplicative_expression,
        source_token_details,
    )
}

fn parse_multiplicative_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left * right, left / right
    parse_binary_expression(
        &vec![Token::Asterisk, Token::Slash],
        parse_optional_or_expression,
        source_token_details,
    )
}

fn parse_optional_or_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left ?? right
    parse_binary_expression(
        &vec![Token::OptionalOr],
        parse_optional_and_expression,
        source_token_details,
    )
}

fn parse_optional_and_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left >> right
    parse_binary_expression(
        &vec![Token::OptionalAnd],
        parse_combine_expression,
        source_token_details,
    )
}

fn parse_combine_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left & right
    // 结合方向：从右向左
    parse_right_2_left_binary_expression(
        &Token::Combine,
        parse_cast_expression,
        source_token_details,
    )
}

fn parse_cast_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 object^
    let (left, post_expression) = parse_negative_expression(source_token_details)?;

    if is_token(&Token::Cast, post_expression) {
        let post_consume_token_operator = consume_token(&Token::Cast, post_expression)?;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Cast,
                operand: Box::new(left),
                range: new_range(),
            }),
            post_consume_token_operator,
        ))
    } else {
        Ok((left, post_expression))
    }
}

fn parse_negative_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 -object
    if is_token(&Token::Minus, source_token_details) {
        let post_consume_token_operator = consume_token(&Token::Cast, source_token_details)?;
        let (left, post_expression) = parse_unwrap_expression(post_consume_token_operator)?;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Minus,
                operand: Box::new(left),
                range: new_range(),
            }),
            post_expression,
        ))
    } else {
        parse_unwrap_expression(source_token_details)
    }
}

fn parse_unwrap_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 object?
    let (left, post_expression) = parse_function_call_expression(source_token_details)?;

    if is_token(&Token::Unwrap, post_expression) {
        let post_consume_token_operator = consume_token(&Token::Unwrap, post_expression)?;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Unwrap,
                operand: Box::new(left),
                range: new_range(),
            }),
            post_consume_token_operator,
        ))
    } else {
        Ok((left, post_expression))
    }
}

fn parse_function_call_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 函数调用表达式
    // - 被调用者必须是一个标识符、一个对象的属性值或索引值；
    // - 被调用者也可以是一个用括号包围起来的表达式或者表达式块，只要是返回函数即可；
    // - 允许连续调用。
    //
    // foo(...)
    // foo.bar(...)
    // foo[0](...)
    // (foo & bar)(...) // 被调用者是一个括号包围起来的表达式
    // foo(...)(...)    // 连续调用

    let mut token_details = source_token_details;
    let (mut object, post_member_expression) = parse_member_or_slice_expression(token_details)?;

    token_details = post_member_expression;

    loop {
        if is_token(&Token::LeftParen, token_details) {
            let (arguments, post_arguments) = continue_parse_arguments(token_details)?;
            object = Expression::FunctionCallExpression(FunctionCallExpression {
                callee: Box::new(object),
                arguments: arguments,
                range: new_range(),
            });

            token_details = post_arguments;
        } else {
            break;
        }
    }

    Ok((object, token_details))
}

fn continue_parse_arguments(
    source_token_details: &[TokenDetail],
) -> Result<(Vec<Argument>, &[TokenDetail]), Error> {
    // (value)
    // (value1, value2)
    // (value1, value2,) // 参数列表末尾也允许有逗号
    //
    // (
    //    value1,
    //    name2=value2,
    // )                 // 参数列表也可以分多行写
    //
    // (value1, name2=value2, name3=value3)
    // ^
    // |--- 当前处于这个位置

    let mut token_details = source_token_details;
    let mut arguments: Vec<Argument> = vec![];
    let mut is_expected_end = false; // 标记当前是否处于一心找映射表结束的状态

    // 消除左括号 `(`
    token_details = consume_token(&Token::LeftParen, token_details)?;

    // 消除左括号 `(` 后面的空行
    token_details = skip_new_lines(token_details);

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if first.token == Token::RightParen {
                    // 找到了结束符号 `)`，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前的状态是一心寻找结束符号 `)`
                        return Err(Error::ParserError(
                            "expected the right paren symbol \")\"".to_string(),
                        ));
                    } else {
                        // 当前是 `key = value` 表达式
                        // 注意其中的 `key` 部分是可选的。

                        let (part_one, post_part_one) = parse_expression(token_details)?;

                        let post_one_argument = if is_token(&Token::Assign, post_part_one) {
                            // 当前存在 `key` 部分

                            // 检查 name 是否 identifier
                            if let Expression::Identifier(Identifier { name, .. }) = part_one {
                                // 消除赋值符号 `=`
                                let post_consume_assign =
                                    consume_token(&Token::Assign, post_part_one)?;

                                // 消除赋值符号 `=` 后面的空行
                                let post_consume_new_lines_after_equal =
                                    skip_new_lines(post_consume_assign);

                                let (value_expression, post_value_expression) =
                                    parse_expression(post_consume_new_lines_after_equal)?;

                                // 构造 Argument
                                let argument = Argument {
                                    name: Some(name),
                                    value: Box::new(value_expression),
                                    range: new_range(),
                                };

                                arguments.push(argument);
                                post_value_expression
                            } else {
                                // 参数名称不正确
                                return Err(Error::ParserError(
                                    "invalid argument name".to_string(),
                                ));
                            }
                        } else {
                            // 当前不存在 `key` 部分

                            // 构造 Argument
                            let argument = Argument {
                                name: None,
                                value: Box::new(part_one),
                                range: new_range(),
                            };

                            arguments.push(argument);

                            post_part_one
                        };

                        // 如果接下来是逗号，表明还有下一项，否则表示后面没有更多项目
                        let post_consume_comma = if is_token(&Token::Comma, post_one_argument) {
                            consume_token(&Token::Comma, post_one_argument)?
                        } else {
                            // 后面没有更多的参数项了
                            is_expected_end = true;
                            post_one_argument
                        };

                        // 消除一项参数后面的空行
                        let post_consume_new_lines = skip_new_lines(post_consume_comma);
                        post_consume_new_lines
                    }
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected the right paren symbol \")\"".to_string(),
                ));
            }
        }
    }

    // 消除右括号 `)`
    token_details = consume_token(&Token::RightParen, token_details)?;

    Ok((arguments, token_details))
}

fn parse_member_or_slice_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 对象的成员（包括属性和索引）以及对象的切片，有相似的结构，
    // 且优先级相同：
    //
    // object["foo"]
    // object[0]
    // object[0..10]
    // object[0..=9]
    // object.name
    // object.1
    // object[other[1]][2]
    // object.name.subname

    let mut token_details = source_token_details;
    let (mut object, post_expression) = parse_constructor_expression(token_details)?;

    token_details = post_expression;

    loop {
        if is_token(&Token::LeftBracket, token_details) {
            // 找到符号 `[`

            let (index_or_slice, post_index_or_slice) =
                continue_parse_index_or_slice(token_details)?;

            // 将解析好的对象重新赋值回对象，因为对象的成员（属性或索引）和切片会连续出现，
            // 且都遵循从左向右的结合顺序。
            object = Expression::MemberExpression(MemberExpression::Index(MemberIndex {
                object: Box::new(object),
                index: Box::new(index_or_slice),
                range: new_range(),
            }));

            token_details = post_index_or_slice;
        } else if is_token_ignore_new_lines(&Token::Dot, token_details) {
            // 找到符号 `.`

            // 消除符号 `.` 前的空行以及符号 `.`
            let post_dot = skip_new_lines_and_consume_token(&Token::Dot, token_details)?;

            let (property, post_property) = parse_primary_expression(post_dot)?;

            // 对象的 `属性` 只允许 identifier 和 integer 两种
            match property {
                Expression::Identifier(_) | Expression::Literal(Literal::Integer(_)) => {
                    // 将解析好的对象重新赋值回对象，因为对象的成员（属性或索引）和切片会连续出现，
                    // 且都遵循从左向右的结合顺序。
                    object =
                        Expression::MemberExpression(MemberExpression::Property(MemberProperty {
                            object: Box::new(object),
                            property: Box::new(property),
                            range: new_range(),
                        }));

                    token_details = post_property;
                }
                _ => {
                    return Err(Error::ParserError("invalid property name".to_string()));
                }
            }
        } else {
            break;
        }
    }

    Ok((object, token_details))
}

fn continue_parse_index_or_slice(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // object["foo"]
    // object[0]
    // object[0..10]
    // object[0..=9]
    //       ^
    //       |--- 当前所处的位置

    let mut token_details = source_token_details;

    // 消除符号 `[`
    token_details = consume_token(&Token::LeftBracket, token_details)?;
    // 消除符号 `[` 后面的空行
    token_details = skip_new_lines(token_details);

    let (mut index_or_slice_expression, post_expression) = parse_expression(token_details)?;

    // 检查是否存在 `范围表达式`
    token_details = if is_token(&Token::Interval, post_expression)
        || is_token(&Token::IntervalInclusive, post_expression)
    {
        let (is_inclusive, optional_to_expression, post_continue_parse_interval) =
            continue_parse_interval(post_expression)?;

        index_or_slice_expression = Expression::Interval(Interval {
            is_inclusive,
            from: Box::new(index_or_slice_expression),
            to: match optional_to_expression {
                Some(end_expression) => Some(Box::new(end_expression)),
                None => None,
            },
            range: new_range(),
        });

        post_continue_parse_interval
    } else {
        post_expression
    };

    // 消除符号 `]`
    token_details = consume_token(&Token::RightBracket, token_details)?;

    Ok((index_or_slice_expression, token_details))
}

fn parse_constructor_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 `通过花括号` 实例化结构体的表达式
    // object {name: vale, ...}

    let (object, post_expression) = parse_primary_expression(source_token_details)?;
    if is_token(&Token::LeftBrace, post_expression) {
        let (initializer, post_continue_parse_map) = continue_parse_map(post_expression)?;

        if let Expression::Identifier(identifier) = object {
            let exp = Expression::ConstructorExpression(ConstructorExpression {
                object: identifier,
                value: initializer,
                range: new_range(),
            });

            Ok((exp, post_continue_parse_map))
        } else {
            Err(Error::ParserError("invalid constructor object".to_string()))
        }
    } else {
        Ok((object, post_expression))
    }
}

// PrimaryExpression
//  : Fn
//  | Tuple/Parenthesized
//  | List
//  | Map
//  | Identifier
//  | Literal
//  ;
fn parse_primary_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 注：
    // 由于范围表达式 `A..B`、`A..=B` 和省略号表达式 `...A` 仅存在于
    // 元组、列表、映射表、切片等场合，所以不单独解析，而是在解析以上节点
    // 时同时解析。
    match source_token_details.first() {
        Some(first) => match first.token {
            Token::Fn => parse_anonymous_function(source_token_details),
            Token::LeftParen => parse_tuple_or_parenthesized(source_token_details),
            Token::LeftBracket => parse_list(source_token_details),
            Token::LeftBrace => parse_map(source_token_details),
            Token::Exclamation => parse_prefix_identifier(source_token_details), // 函数的前置调用
            Token::Identifier(_) => parse_identifier(source_token_details),
            Token::Sign => parse_sign_expression(source_token_details),
            _ => {
                let (literal, post_literal) = parse_literal(source_token_details)?;
                Ok((Expression::Literal(literal), post_literal))
            }
        },
        None => Err(Error::ParserError(
            "expected primary expression".to_string(),
        )),
    }
}

fn parse_anonymous_function(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 匿名函数
    // 匿名函数没有函数名称、不支持泛型、不支持默认值、参数和返回值可省略数据类型
    //
    // fn (Int a, Int b) type Int = ...
    // fn (Int a, Int b) type Int which ... = ...
    // fn (a, b) = ...
    // fn x = ...            // 当参数只有一个，且省略数据类型时，可以省略参数的括号
    // fn x {...}            // 匿名函数的主体也有可能是 `隠式 do 表达式`

    let mut token_details = source_token_details;

    let mut parameters: Vec<AnonymousParameter> = vec![];
    let mut return_data_type: Option<DataType> = None;
    let mut which_entries: Vec<WhichEntry> = vec![];

    let mut is_expected_end = false; // 标记当前是否处于寻找参数列表结束符号 `)` 的状态

    token_details = consume_token(&Token::Fn, token_details)?;
    token_details = skip_new_lines(token_details); // 关键字后允许换行

    // 解析参数列表
    let post_parameters = match token_details.split_first() {
        Some((maybe_left_paren, post_left_paren)) if maybe_left_paren.token == Token::LeftParen => {
            // 参数列表有括号包围

            // 消除符号 `(` 后面的空行
            token_details = skip_new_lines(post_left_paren);

            // 解析参数列表
            loop {
                token_details = match token_details.first() {
                    Some(first) => {
                        if first.token == Token::RightParen {
                            // 找到了结束符号 `)`，退出循环
                            break;
                        } else {
                            if is_expected_end {
                                // 当前的状态是一心寻找结束符号
                                return Err(Error::ParserError(
                                    "expected the right paren symbol \")\"".to_string(),
                                ));
                            } else {
                                // 先尝试寻找参数的数据类型
                                let (part_one, post_part_one) = parse_expression(token_details)?;

                                let post_one_parameter = match post_part_one.split_first() {
                                    Some((maybe_comma_or_right_paren, _))
                                        if maybe_comma_or_right_paren.token == Token::Comma
                                            || maybe_comma_or_right_paren.token
                                                == Token::RightParen =>
                                    {
                                        // 当前参数无数据类型
                                        if let Expression::Identifier(Identifier { name, .. }) =
                                            part_one
                                        {
                                            parameters.push(AnonymousParameter {
                                                data_type: None,
                                                name: name,
                                                range: new_range(),
                                            });
                                            post_part_one
                                        } else {
                                            return Err(Error::ParserError(
                                                "invalid anonymous function parameter name"
                                                    .to_string(),
                                            ));
                                        }
                                    }
                                    Some((
                                        TokenDetail {
                                            token: Token::Identifier(name),
                                            ..
                                        },
                                        post_part_two,
                                    )) => {
                                        // 当前参数有数据类型
                                        let data_type = convert_expression_to_data_type(part_one)?;
                                        parameters.push(AnonymousParameter {
                                            data_type: Some(data_type),
                                            name: name.clone(),
                                            range: new_range(),
                                        });
                                        post_part_two
                                    }
                                    _ => {
                                        return Err(Error::ParserError(
                                            "incomplete anonymous function parameter".to_string(),
                                        ));
                                    }
                                };

                                // 消除逗号
                                let post_consume_comma =
                                    if is_token(&Token::Comma, post_one_parameter) {
                                        consume_token(&Token::Comma, post_one_parameter)?
                                    } else {
                                        // 设置标记，表示如果项目后面没有逗号，则表示当前已经是最后一项
                                        // 后面只能允许列表结束
                                        is_expected_end = true;
                                        post_one_parameter
                                    };

                                // 消除空行
                                let post_consume_new_lines = skip_new_lines(post_consume_comma);
                                post_consume_new_lines
                            }
                        }
                    }
                    None => {
                        return Err(Error::ParserError(
                            "expected the right paren symbol \")\"".to_string(),
                        ));
                    }
                }
            }

            // 消除右括号
            consume_token(&Token::RightParen, token_details)?
        }
        Some((
            TokenDetail {
                token: Token::Identifier(name),
                ..
            },
            post_left_paren,
        )) => {
            // 参数列表只有一个参数，且无括号包围
            parameters.push(AnonymousParameter {
                data_type: None,
                name: name.clone(),
                range: new_range(),
            });
            post_left_paren
        }
        _ => {
            return Err(Error::ParserError(
                "expected anonymous function parameter".to_string(),
            ));
        }
    };

    // 消除参数列表后面
    token_details = skip_new_lines(post_parameters);

    loop {
        // 尝试解析 type, which 等从属表达式
        token_details = match token_details.first() {
            Some(t) if t.token == Token::Type => {
                let (data_type, post_data_type_expression) =
                    continue_parse_type_expression(token_details)?;

                return_data_type = Some(data_type);

                // 消除从属表达式后面的空行
                skip_new_lines(post_data_type_expression)
            }
            Some(t) if t.token == Token::Which => {
                let (entries, post_which_expression) =
                    continue_parse_which_expression(token_details)?;

                which_entries = entries;

                // 消除从属表达式后面的空行
                skip_new_lines(post_which_expression)
            }
            _ => {
                break;
            }
        }
    }

    // 消除赋值符号（如果存在的话）
    let post_assignment = if is_token(&Token::Assign, token_details) {
        let post_assignment_token = consume_token(&Token::Assign, token_details)?;
        // 消除空行
        skip_new_lines(post_assignment_token)
    } else {
        token_details
    };

    // 解析函数主体
    let (body, post_body) = continue_parse_expression_block_or_single_expression(post_assignment)?;

    // 构造匿名函数对象
    let anonymous_function = AnonymousFunction {
        parameters: parameters,
        return_data_type: return_data_type,
        which_entries: which_entries,
        // where_exp: where_exp,
        body: Box::new(body),
        range: new_range(),
    };

    Ok((Expression::AnonymousFunction(anonymous_function), post_body))
}

fn convert_expression_to_data_type(exp: Expression) -> Result<DataType, Error> {
    match exp {
        Expression::Identifier(identifier) => Ok(DataType::Identifier(identifier)),
        Expression::Sign(sign) => Ok(DataType::Sign(sign)),
        Expression::Tuple(tuple) => Ok(DataType::Tuple(tuple)),
        _ => Err(Error::ParserError(
            "invalid anonymous function parameter data type".to_string(),
        )),
    }
}

fn parse_list(source_token_details: &[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error> {
    // list
    //
    // e.g.
    // [123, 345, 567,]
    // [123, 345, 567]   // 末尾逗号可省略
    //
    // [
    //    123,  // 换行时，项目之间的逗号不可以省略
    //    456,
    //    678
    // ]
    //
    // [1..10]
    // [1..=9]
    // [1,3..10]

    let mut token_details = source_token_details;

    let mut expressions: Vec<Expression> = vec![];
    let mut is_expected_end = false; // 标记当前是否处于一心寻找结束符的状态

    // 消除左中括号（方括号） `[`
    token_details = consume_token(&Token::LeftBracket, token_details)?;

    // 消除左中括号（方括号） `[` 后面的空行
    token_details = skip_new_lines(token_details);

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if first.token == Token::RightBracket {
                    // 找到了结束符号 `]`，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前的状态是一心寻找结束符号 `]`
                        return Err(Error::ParserError(
                            "expected the right bracket symbol \"]\"".to_string(),
                        ));
                    } else {
                        // 先检查是否 `省略符表达式`
                        if first.token == Token::Ellipsis {
                            // 当前是 `省略符表达式`
                            let (ellipsis, post_ellipsis) = continue_parse_ellipsis(token_details)?;
                            expressions.push(Expression::Ellipsis(ellipsis));
                            is_expected_end = true; // 设置标记，`省略符表达式` 后面只能允许列表结束

                            // 消除逗号 `,`
                            let post_consume_comma = if is_token(&Token::Comma, post_ellipsis) {
                                consume_token(&Token::Comma, post_ellipsis)?
                            } else {
                                post_ellipsis
                            };

                            // 消除逗号 `,` 后面的空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        } else {
                            // 当前是普通表达式或者 `范围表达式`
                            let (expression, post_expression) = parse_expression(token_details)?;

                            let post_check_interval = if is_token(&Token::Interval, post_expression)
                                || is_token(&Token::IntervalInclusive, post_expression)
                            {
                                // 当前是 `范围表达式`
                                let (
                                    is_inclusive,
                                    optional_to_expression,
                                    post_continue_parse_interval,
                                ) = continue_parse_interval(post_expression)?;

                                let interval_expression = Expression::Interval(Interval {
                                    is_inclusive,
                                    from: Box::new(expression),
                                    to: match optional_to_expression {
                                        Some(end_expression) => Some(Box::new(end_expression)),
                                        None => None,
                                    },
                                    range: new_range(),
                                });

                                is_expected_end = true; // 设置标记，`范围表达式` 后面只能允许列表结束

                                expressions.push(interval_expression);
                                post_continue_parse_interval
                            } else {
                                // 当前是普通表达式
                                expressions.push(expression);
                                post_expression
                            };

                            // 消除逗号 `,`
                            let post_consume_comma = if is_token(&Token::Comma, post_check_interval)
                            {
                                consume_token(&Token::Comma, post_check_interval)?
                            } else {
                                // 设置标记，表示如果项目后面没有逗号，则表示当前已经是最后一项
                                // 后面只能允许列表结束
                                is_expected_end = true;
                                post_check_interval
                            };

                            // 消除逗号 `,` 后面的空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        }
                    }
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected the right bracket symbol \")\"".to_string(),
                ))
            }
        }
    }

    // 消除右中括号（方括号） `]`
    token_details = consume_token(&Token::RightBracket, token_details)?;

    Ok((
        Expression::List(List {
            elements: expressions,
            range: new_range(),
        }),
        token_details,
    ))
}

fn parse_tuple_or_parenthesized(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // tuple or parenthesized
    //
    // e.g.
    //
    // tuple:
    //
    // ()
    // (one,)              // 单独一个元素的末尾逗号不可省略
    // (one, two, )        // 末尾逗号可以省略
    // (one, two, ...)     //
    // (one, two, ...rest) //
    //
    // (
    //    123,  // 换行时，项目之间的逗号不可以省略
    //    456,
    //    678
    // )
    //
    // parenthesized:
    //
    // (expression)

    let mut token_details = source_token_details;

    let mut expressions: Vec<Expression> = vec![];
    let mut is_tuple = false; // 标记当前是否元组（而不是表达式括号运算）
    let mut is_expected_end = false; // 标记当前是否处于一心寻找结束的状态

    // 消除左括号 `(`
    token_details = consume_token(&Token::LeftParen, token_details)?;

    // 消除左括号 `(` 后面的空行
    token_details = skip_new_lines(token_details);

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if first.token == Token::RightParen {
                    // 找到了结束符号 `)`，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前的状态是一心寻找结束符号 `)`
                        return Err(Error::ParserError(
                            "expected the right paren symbol \")\"".to_string(),
                        ));
                    } else {
                        // 先检查是否 `省略符表达式`
                        if first.token == Token::Ellipsis {
                            // 当前是 `省略符表达式`
                            let (ellipsis, post_ellipsis) = continue_parse_ellipsis(token_details)?;
                            expressions.push(Expression::Ellipsis(ellipsis));
                            is_expected_end = true; // 设置标记，`省略符表达式` 后面只能允许列表结束

                            // 消除逗号 `,`
                            let post_consume_comma = if is_token(&Token::Comma, post_ellipsis) {
                                consume_token(&Token::Comma, post_ellipsis)?
                            } else {
                                post_ellipsis
                            };

                            // 消除逗号 `,` 后面的空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        } else {
                            // 当前是普通表达式
                            let (expression, post_expression) = parse_expression(token_details)?;
                            expressions.push(expression);

                            // 消除逗号 `,`
                            let post_consume_comma = if is_token(&Token::Comma, post_expression) {
                                // 检测到逗号，设置标记，表明当前表达式是元组而非括号表达式
                                is_tuple = true;
                                consume_token(&Token::Comma, post_expression)?
                            } else {
                                // 设置标记，表示如果项目后面没有逗号，则表示当前已经是最后一项
                                // 后面只能允许列表结束
                                is_expected_end = true;
                                post_expression
                            };

                            // 消除逗号 `,` 后面的空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        }
                    }
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected the right paren symbol \")\"".to_string(),
                ))
            }
        }
    }

    // 消除右括号 `)`
    token_details = consume_token(&Token::RightParen, token_details)?;

    if expressions.len() == 0 {
        // 空元组
        Ok((
            Expression::Tuple(Tuple {
                elements: vec![],
                range: new_range(),
            }),
            token_details,
        ))
    } else {
        if is_tuple {
            // 元组
            Ok((
                Expression::Tuple(Tuple {
                    elements: expressions,
                    range: new_range(),
                }),
                token_details,
            ))
        } else {
            // 普通的括号表达式
            Ok((expressions[0].clone(), token_details))
        }
    }
}

fn continue_parse_ellipsis(
    source_token_details: &[TokenDetail],
) -> Result<(Ellipsis, &[TokenDetail]), Error> {
    // ...
    // ..._
    // ...abc
    // ^  ^--- identifier
    // |------ ellipsis，当前处于这个 token

    // 消除省略号 `...`
    let post_consume_token_ellipsis = consume_token(&Token::Ellipsis, source_token_details)?;

    if let Some((
        TokenDetail {
            token: Token::Identifier(name),
            ..
        },
        post_consume_token_identifier,
    )) = post_consume_token_ellipsis.split_first()
    {
        // 省略号 `...` 后面有标识符
        Ok((
            Ellipsis {
                name: Some(name.clone()),
                range: new_range(),
            },
            post_consume_token_identifier,
        ))
    } else {
        // 省略号 `...` 后面无标识符
        Ok((
            Ellipsis {
                name: None,
                range: new_range(),
            },
            post_consume_token_ellipsis,
        ))
    }
}

// 解析 `范围表达式`
// 返回 (`to` 是否闭区间, `to` 表达式, 剩余的 token)
fn continue_parse_interval(
    source_token_details: &[TokenDetail],
) -> Result<(bool, Option<Expression>, &[TokenDetail]), Error> {
    // exp1..=
    // exp1..=exp2
    // exp1..
    // exp1..exp2
    // ^   ^ ^--- expression (可选的)
    // |   |----- interval 当前处于这个 token
    // |--------- expression

    let is_inclusive = is_token(&Token::IntervalInclusive, source_token_details);
    let operator_token = if is_inclusive {
        Token::IntervalInclusive
    } else {
        Token::Interval
    };

    // 消除符号 ".." 或者 "..="
    let post_consume_token_interval = consume_token(&operator_token, source_token_details)?;

    // 消除符号 ".."  或者 "..=" 后面的空行
    let post_new_lines = skip_new_lines(post_consume_token_interval);

    match post_new_lines.first() {
        Some(TokenDetail { token, .. })
            if (*token == Token::Comma || *token == Token::RightBracket) =>
        {
            // 遇到了逗号或者右中括号（方括号）
            if is_inclusive {
                // 对于闭区间的范围表达式，`to` 部分是不能省略的。
                Err(Error::ParserError(
                    "expected inclusive range end".to_string(),
                ))
            } else {
                // 当前范围表达式缺省了 `to` 部分。
                Ok((is_inclusive, None, post_new_lines))
            }
        }
        _ => {
            // 解析 `to` 部分表达式
            let (to_expression, post_to_expression) = parse_expression(post_new_lines)?;
            Ok((is_inclusive, Some(to_expression), post_to_expression))
        }
    }
}

fn parse_map(source_token_details: &[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error> {
    let (map, post_continue_parse_map) = continue_parse_map(source_token_details)?;
    Ok((Expression::Map(map), post_continue_parse_map))
}

fn continue_parse_map(
    source_token_details: &[TokenDetail],
) -> Result<(Map, &[TokenDetail]), Error> {
    // map
    //
    // e.g.
    // {name: value, name: value} // 项目之间使用逗号分隔
    // {id, name, ...rest}        // 末尾逗号可以省略
    // {
    //    id: value,
    //    name: value    // 换行时，项目之间的逗号 **可以** 省略
    //    checkd: value  // 这种格式对于同样使用花括号作为主体的 map/branch/match 三种表达式都保持一致
    // }

    let mut token_details = source_token_details;
    let mut entries: Vec<MapEntry> = vec![];
    let mut is_expected_end = false; // 标记当前是否处于一心找映射表结束的状态

    // 消除左花括号 `{`
    token_details = consume_token(&Token::LeftBrace, token_details)?;

    // 消除左花括号 `{` 后面的空行
    token_details = skip_new_lines(token_details);

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if first.token == Token::RightBrace {
                    // 找到了结束符号 `}`，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前的状态是一心寻找结束符号 `}`
                        return Err(Error::ParserError(
                            "expected the right brace symbol \"}\"".to_string(),
                        ));
                    } else {
                        // 先检查是否 `省略符表达式`
                        if first.token == Token::Ellipsis {
                            // 当前是 `省略符表达式`
                            let (ellipsis, post_ellipsis) = continue_parse_ellipsis(token_details)?;

                            // `省略表达式` 以 `key` 添加到项目里
                            entries.push(MapEntry {
                                key: Box::new(Expression::Ellipsis(ellipsis)),
                                value: None,
                                range: new_range(),
                            });
                            is_expected_end = true; // 设置标记，`省略符表达式` 后面只能允许列表结束

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_ellipsis) {
                                consume_token(&Token::Comma, post_ellipsis)?
                            } else {
                                post_ellipsis
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        } else {
                            // 当前是 `key: value` 表达式
                            // 注意其中的 `value` 部分是可选的。

                            let (expression, post_key_expression) =
                                parse_expression(token_details)?;

                            let post_one_entry = if is_token(&Token::Colon, post_key_expression) {
                                // 当前存在 `value` 部分

                                // 消除冒号 `:`
                                let post_consume_colon =
                                    consume_token(&Token::Colon, post_key_expression)?;

                                // 消除冒号 `:` 后面的空行
                                let post_consume_new_lines_after_colon =
                                    skip_new_lines(post_consume_colon);

                                let (value_expression, post_value_expression) =
                                    parse_expression(post_consume_new_lines_after_colon)?;

                                // 构造 MapEntry
                                let entry = MapEntry {
                                    key: Box::new(expression),
                                    value: Some(Box::new(value_expression)),
                                    range: new_range(),
                                };

                                entries.push(entry);
                                post_value_expression
                            } else {
                                // 当前不存在 `value` 部分

                                // 构造 MapEntry
                                let entry = MapEntry {
                                    key: Box::new(expression),
                                    value: None,
                                    range: new_range(),
                                };

                                entries.push(entry);
                                post_key_expression
                            };

                            // 如果接下来是：
                            // - 逗号
                            // - 逗号+空行
                            // - 空行
                            //
                            // 表明还有下一项，否则表示后面没有更多项目

                            let post_consume_comma = match post_one_entry.split_first() {
                                Some((first, rest)) if first.token == Token::Comma => {
                                    // 消除逗号
                                    rest
                                }
                                Some((first, _)) if first.token == Token::NewLine => {
                                    // 等接下来的代码来统一来消除空行
                                    post_one_entry
                                }
                                _ => {
                                    // 没有下一项了，标记映射表的已经到达末尾
                                    is_expected_end = true;
                                    post_one_entry
                                }
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        }
                    }
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected the right brace symbol \"}\"".to_string(),
                ));
            }
        }
    }

    // 消除右花括号 `}`
    token_details = consume_token(&Token::RightBrace, token_details)?;

    Ok((
        Map {
            elements: entries,
            range: new_range(),
        },
        token_details,
    ))
}

fn parse_prefix_identifier(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // prefix identifier
    let post_consume_token_exclamation = consume_token(&Token::Exclamation, source_token_details)?;

    let (identifier, post_continue_parse_identifier) =
        continue_parse_identifier(post_consume_token_exclamation)?;

    Ok((
        Expression::PrefixIdentifier(PrefixIdentifier {
            identifier: identifier,
            range: new_range(),
        }),
        post_continue_parse_identifier,
    ))
}

fn parse_identifier(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // identifier
    //
    // One::Two::Three::Name
    // Name<T>
    // Name<T, E>
    let (identifier, post_continue_parse_identifier) =
        continue_parse_identifier(source_token_details)?;

    Ok((
        Expression::Identifier(identifier),
        post_continue_parse_identifier,
    ))
}

fn continue_parse_identifier(
    source_token_details: &[TokenDetail],
) -> Result<(Identifier, &[TokenDetail]), Error> {
    // identifier
    //
    // e.g.
    // One::Two::Three::Name
    let mut token_details = source_token_details;
    let mut names: Vec<String> = vec![];

    if let Some((
        TokenDetail {
            token: Token::Identifier(name),
            ..
        },
        rest,
    )) = token_details.split_first()
    {
        // 获取第一个 identifier
        names.push(name.clone());
        token_details = rest;

        // 获取其余的 identifier
        loop {
            token_details = match token_details.split_first() {
                Some((first, post_token_separator)) if first.token == Token::Separator => {
                    // 检测到 namespace path 分隔符 `::`
                    if let Some((
                        TokenDetail {
                            token: Token::Identifier(name),
                            ..
                        },
                        post_token_identifier,
                    )) = post_token_separator.split_first()
                    {
                        // 检测到一个 identifier
                        names.push(name.clone());
                        post_token_identifier
                    } else {
                        // 在 namespace path 分隔符 `::` 后面必须是一个 identifier
                        return Err(Error::ParserError("expected identifier".to_string()));
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    if names.len() == 0 {
        return Err(Error::ParserError("expected identifier".to_string()));
    }

    let mut generics: Vec<DataType> = vec![];

    // 解析泛型
    if is_token(&Token::LessThan, token_details) {
        let (data_types, post_generics) = continue_parse_generic_names(token_details)?;
        generics = data_types;
        token_details = post_generics;
    }

    let len = names.len();
    Ok((
        Identifier {
            dirs: names[..len - 1].iter().map(|n| n.clone()).collect(),
            name: names[len - 1].clone(),
            generics: generics,
            range: new_range(),
        },
        token_details,
    ))
}

fn parse_sign_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 函数签名
    // 签名没有名称、参数名称可省略，不支持默认值
    //
    // `sign (Int x, Int y) type Int`
    // `sign <T, E> (T x, E y) type T`
    // `sign (T a, String s) type T which {T: Int}`

    let mut token_details = source_token_details;

    let mut generics: Vec<DataType> = vec![];
    let mut parameters: Vec<SignParameter> = vec![];
    let mut return_data_type: Option<Box<DataType>> = None;
    let mut which_entries: Vec<WhichEntry> = vec![];

    let mut is_expected_end = false; // 标记当前是否处于寻找参数列表结束符号 `)` 的状态

    token_details = consume_token(&Token::Sign, token_details)?;
    token_details = skip_new_lines(token_details); // 关键字后允许换行

    // 解析泛型
    if is_token(&Token::LessThan, token_details) {
        let (data_types, post_generics) = continue_parse_generic_names(token_details)?;
        generics = data_types;

        // 消除符号 `>` 后面的空行
        token_details = skip_new_lines(post_generics);
    }

    // 解析参数列表
    let post_parameters = match token_details.split_first() {
        Some((maybe_left_paren, post_left_paren)) if maybe_left_paren.token == Token::LeftParen => {
            // 消除符号 `(` 后面的空行
            token_details = skip_new_lines(post_left_paren);

            // 解析参数列表
            loop {
                token_details = match token_details.first() {
                    Some(first) => {
                        if first.token == Token::RightParen {
                            // 找到了结束符号 `)`，退出循环
                            break;
                        } else {
                            if is_expected_end {
                                // 当前的状态是一心寻找结束符号
                                return Err(Error::ParserError(
                                    "expected the right paren symbol \")\"".to_string(),
                                ));
                            } else {
                                // 获取参数的数据类型
                                let (data_type_expression, post_data_type_expression) =
                                    parse_expression(token_details)?;
                                let data_type =
                                    convert_expression_to_data_type(data_type_expression)?;

                                let post_one_parameter =
                                    match post_data_type_expression.split_first() {
                                        Some((maybe_comma_or_right_paren, _))
                                            if maybe_comma_or_right_paren.token == Token::Comma
                                                || maybe_comma_or_right_paren.token
                                                    == Token::RightParen =>
                                        {
                                            // 当前参数无名称
                                            parameters.push(SignParameter {
                                                data_type: data_type,
                                                name: None,
                                                range: new_range(),
                                            });
                                            post_data_type_expression
                                        }
                                        Some((
                                            TokenDetail {
                                                token: Token::Identifier(name),
                                                ..
                                            },
                                            post_name,
                                        )) => {
                                            // 当前参数有名称
                                            parameters.push(SignParameter {
                                                data_type: data_type,
                                                name: Some(name.clone()),
                                                range: new_range(),
                                            });
                                            post_name
                                        }
                                        _ => {
                                            return Err(Error::ParserError(
                                                "incomplete function parameter".to_string(),
                                            ));
                                        }
                                    };

                                // 消除逗号
                                let post_consume_comma =
                                    if is_token(&Token::Comma, post_one_parameter) {
                                        consume_token(&Token::Comma, post_one_parameter)?
                                    } else {
                                        // 设置标记，表示如果项目后面没有逗号，则表示当前已经是最后一项
                                        // 后面只能允许列表结束
                                        is_expected_end = true;
                                        post_one_parameter
                                    };

                                // 消除空行
                                let post_consume_new_lines = skip_new_lines(post_consume_comma);
                                post_consume_new_lines
                            }
                        }
                    }
                    None => {
                        return Err(Error::ParserError(
                            "expected the right paren symbol \")\"".to_string(),
                        ));
                    }
                }
            }

            // 消除右括号
            consume_token(&Token::RightParen, token_details)?
        }
        _ => {
            return Err(Error::ParserError(
                "expected anonymous function parameter".to_string(),
            ));
        }
    };

    // 消除参数列表后面
    token_details = skip_new_lines(post_parameters);

    loop {
        // 尝试解析 type, which 等从属表达式
        token_details = match token_details.first() {
            Some(t) if t.token == Token::Type => {
                let (data_type, post_data_type_expression) =
                    continue_parse_type_expression(token_details)?;

                return_data_type = Some(Box::new(data_type));

                // 消除从属表达式后面的空行
                skip_new_lines(post_data_type_expression)
            }
            Some(t) if t.token == Token::Which => {
                let (entries, post_which_expression) =
                    continue_parse_which_expression(token_details)?;

                which_entries = entries;

                // 消除从属表达式后面的空行
                skip_new_lines(post_which_expression)
            }
            _ => {
                break;
            }
        }
    }

    // 构造函数签名对象
    let sign = Sign {
        parameters: parameters,
        return_data_type: return_data_type,
        generics: generics,
        which_entries: which_entries,
        range: new_range(),
    };

    Ok((Expression::Sign(sign), token_details))
}

// Literal
//  : Integer
//  | Float
//  | Complex
//  | Bit
//  | Boolean
//  | Char
//  | GeneralString
//  | TemplateString
//  | HashString
//  | NamedOperator
//  ;

fn parse_literal(source_token_details: &[TokenDetail]) -> Result<(Literal, &[TokenDetail]), Error> {
    match source_token_details.split_first() {
        Some((first, rest)) => match &first.token {
            Token::Integer(v) => match continue_parse_imaginary(rest) {
                // 整数或复数
                Ok((f, post_rest)) => Ok((
                    Literal::Complex(Complex {
                        real: *v as f64,
                        imaginary: f,
                        range: new_range(),
                    }),
                    post_rest,
                )),
                _ => Ok((
                    Literal::Integer(Integer {
                        value: *v,
                        range: new_range(),
                    }),
                    rest,
                )),
            },
            Token::Float(v) => match continue_parse_imaginary(rest) {
                // 浮点数或复数
                Ok((f, post_rest)) => Ok((
                    Literal::Complex(Complex {
                        real: *v,
                        imaginary: f,
                        range: new_range(),
                    }),
                    post_rest,
                )),
                _ => Ok((
                    Literal::Float(Float {
                        value: *v,
                        range: new_range(),
                    }),
                    rest,
                )),
            },
            Token::Imaginary(v) => {
                // 只有单独虚部的复数
                Ok((
                    Literal::Complex(Complex {
                        real: 0f64,
                        imaginary: *v,
                        range: new_range(),
                    }),
                    rest,
                ))
            }
            Token::Bit(width, bytes) => Ok((
                Literal::Bit(Bit {
                    width: *width,
                    bytes: bytes.clone(),
                    range: new_range(),
                }),
                rest,
            )),
            Token::Boolean(v) => Ok((
                Literal::Boolean(Boolean {
                    value: *v,
                    range: new_range(),
                }),
                rest,
            )),
            Token::Char(v) => Ok((
                Literal::Char(Char {
                    value: *v,
                    range: new_range(),
                }),
                rest,
            )),
            Token::GeneralString(v) => Ok((
                Literal::GeneralString(GeneralString {
                    value: v.clone(),
                    range: new_range(),
                }),
                rest,
            )),
            Token::TemplateString(v) => {
                // todo::
                // 这里需要重新 tokenize 模板字符串里面的占位符表达式，
                // 然后重新解析这些表达式
                todo!()
            }
            Token::HashString(v) => Ok((
                Literal::HashString(HashString {
                    value: v.clone(),
                    range: new_range(),
                }),
                rest,
            )),
            Token::NamedOperator(v) => Ok((
                Literal::NamedOperator(NamedOperator {
                    value: v.clone(),
                    range: new_range(),
                }),
                rest,
            )),
            _ => Err(Error::ParserError("unexpected literal".to_string())),
        },
        None => Err(Error::ParserError("expected literal".to_string())),
    }
}

// 尝试解析复数，如果成功则返回虚数及剩余的 token，
// 如果不成功则返回空元
fn continue_parse_imaginary(
    source_token_details: &[TokenDetail],
) -> Result<(f64, &[TokenDetail]), ()> {
    match source_token_details.split_first() {
        Some((first, rest)) if first.token == Token::Plus => match rest.split_first() {
            Some((
                TokenDetail {
                    token: Token::Imaginary(f),
                    ..
                },
                post_rest,
            )) => Ok((*f, post_rest)),
            _ => {
                // 当前表达式并非复数（但不是错误）
                Err(())
            }
        },
        _ => {
            // 当前表达式并非复数（但不是错误）
            Err(())
        }
    }
}

// 跳过空白的行，在 lexer 里产生的 Token 序列当中，有可能存在多行连续的空行，
// 在解析一个statement 之前，或者 expression 之间，需要消除这些空白的前导空行
fn skip_new_lines(source_token_details: &[TokenDetail]) -> &[TokenDetail] {
    let mut token_details = source_token_details;

    loop {
        token_details = match token_details.split_first() {
            Some((first, rest)) if first.token == Token::NewLine => rest,
            _ => {
                break;
            }
        }
    }

    token_details
}

fn skip_new_lines_and_consume_token<'a>(
    expected: &Token,
    source_token_details: &'a [TokenDetail],
) -> Result<&'a [TokenDetail], Error> {
    let post_new_lines = skip_new_lines(source_token_details);
    consume_token(expected, post_new_lines)
}

fn is_token(expected: &Token, source_token_details: &[TokenDetail]) -> bool {
    match source_token_details.first() {
        Some(first) if &first.token == expected => true,
        _ => false,
    }
}

fn is_token_ignore_new_lines(expected: &Token, source_token_details: &[TokenDetail]) -> bool {
    let token_details = skip_new_lines(source_token_details);
    is_token(expected, token_details)
}

fn consume_token<'a>(
    expected: &Token,
    source_token_details: &'a [TokenDetail],
) -> Result<&'a [TokenDetail], Error> {
    match source_token_details.split_first() {
        Some((first, rest)) if &first.token == expected => Ok(rest),
        _ => Err(Error::ParserError(format!(
            "expected the specified symbol \"{}\"",
            expected
        ))),
    }
}

// fn consume_token_if_exists<'a>(
//     expected: &Token,
//     source_token_details: &'a [TokenDetail],
// ) -> &'a [TokenDetail] {
//     match source_token_details.split_first() {
//         Some((first, rest)) if &first.token == expected => rest,
//         _ => source_token_details,
//     }
// }

fn consume_new_line_or_end_of_file(
    source_token_details: &[TokenDetail],
) -> Result<&[TokenDetail], Error> {
    match source_token_details.split_first() {
        Some((first, rest)) => {
            if first.token == Token::NewLine {
                Ok(rest)
            } else {
                Err(Error::ParserError(
                    "expected the new-line symbol".to_string(),
                ))
            }
        }
        None => Ok(source_token_details),
    }
}

fn new_range() -> Range {
    // todo::
    // 各成员的值应该有参数传入
    Range {
        file_id: 0,
        start: 0,
        end: 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            BinaryExpression, BlockExpression, Complex, Ellipsis, Expression, Float, Identifier,
            Integer, Interval, LetExpression, List, Literal, Node, PrefixIdentifier, Program,
            Statement, Tuple,
        },
        error::Error,
        lexer,
        parser::new_range,
        token::Token,
    };

    use super::parse;

    // 辅助函数

    fn new_identifier(name: &str) -> Identifier {
        Identifier {
            dirs: vec![],
            generics: vec![],
            name: name.to_string(),
            range: new_range(),
        }
    }

    fn new_literal_integer(value: i64) -> Literal {
        Literal::Integer(Integer {
            value: value,
            range: new_range(),
        })
    }

    fn parse_from_string(text: &str) -> Result<Node, Error> {
        let token_details = lexer::tokenize(text)?;
        parse(&token_details)
    }

    fn trim_left_margin(s: &str) -> String {
        s.split("\n")
            .map(|s| s.trim_start().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    // literal

    #[test]
    fn test_integer_literal() {
        let n1 = parse_from_string("123").unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Literal(
                    Literal::Integer(Integer {
                        value: 123,
                        range: new_range()
                    })
                ))],
                range: new_range()
            })
        );

        assert_eq!(n1.to_string(), "123\n"); // Statement 以符号 '\n' 结尾
    }

    #[test]
    fn test_float_literal() {
        let n1 = parse_from_string("3.14").unwrap();
        assert_eq!(n1.to_string(), "3.14\n");

        let n2 = parse_from_string("3.14e2").unwrap();
        assert_eq!(n2.to_string(), "314\n");

        let n3 = parse_from_string("3.14e-1").unwrap();
        assert_eq!(n3.to_string(), "0.314\n");
    }

    #[test]
    fn test_complex_literal() {
        let n1 = parse_from_string("3+4i").unwrap();
        assert_eq!(n1.to_string(), "3+4i\n");

        let n2 = parse_from_string("0+2i").unwrap();
        assert_eq!(n2.to_string(), "0+2i\n");

        let n3 = parse_from_string("5i").unwrap();
        assert_eq!(n3.to_string(), "0+5i\n");

        let n4 = parse_from_string("1.414+2.718i").unwrap();
        assert_eq!(n4.to_string(), "1.414+2.718i\n");

        let n5 = parse_from_string("3.14i").unwrap();
        assert_eq!(n5.to_string(), "0+3.14i\n");

        let n6 = parse_from_string("3.14e2i").unwrap();
        assert_eq!(n6.to_string(), "0+314i\n");

        let n7 = parse_from_string("3.14e-1i").unwrap();
        assert_eq!(n7.to_string(), "0+0.314i\n");
    }

    #[test]
    fn test_bit_literal() {
        // todo::
        // let n1 = parse_from_string("16'x08cd").unwrap();
        // assert_eq!(n1.to_string(), "16'x08cd\n");
        //
        // let n2 = parse_from_string("8'b10000001").unwrap();
        // assert_eq!(n2.to_string(), "8'x81\n");
    }

    #[test]
    fn test_boolean_literal() {
        let n1 = parse_from_string("true").unwrap();
        assert_eq!(n1.to_string(), "true\n");

        let n2 = parse_from_string("false").unwrap();
        assert_eq!(n2.to_string(), "false\n");
    }

    #[test]
    fn test_char_literal() {
        let n1 = parse_from_string("'a'").unwrap();
        assert_eq!(n1.to_string(), "'a'\n");

        let n2 = parse_from_string("'文'").unwrap();
        assert_eq!(n2.to_string(), "'文'\n");

        // todo:: 测试转义字符，转义希腊字符
        // todo:: 测试 Unicode
    }

    #[test]
    fn test_general_string_literal() {
        let n1 = parse_from_string("\"abc\"").unwrap();
        assert_eq!(n1.to_string(), "\"abc\"\n");

        let n2 = parse_from_string("\"中文🐱\"").unwrap();
        assert_eq!(n2.to_string(), "\"中文🐱\"\n");

        // 测试多行文本
        let n3 = parse_from_string("\"foo\nbar\n  baz\"").unwrap();
        assert_eq!(n3.to_string(), "\"foo\nbar\n  baz\"\n");

        // todo:: 测试转义字符
    }

    #[test]
    fn test_template_string_literal() {
        // todo::
    }

    #[test]
    fn test_hash_string_literal() {
        let n1 = parse_from_string("#abc").unwrap();
        assert_eq!(n1.to_string(), "#abc\n");

        let n2 = parse_from_string("#foo_bar").unwrap();
        assert_eq!(n2.to_string(), "#foo_bar\n");

        // todo:: 添加中文的支持
        // let n3 = parse_from_string("#中文🐱").unwrap();
        // assert_eq!(n3.to_string(), "#中文🐱\n");
    }

    #[test]
    fn test_named_operator_string_literal() {
        let n1 = parse_from_string(":abc:").unwrap();
        assert_eq!(n1.to_string(), ":abc:\n");

        let n2 = parse_from_string(":foo_bar:").unwrap();
        assert_eq!(n2.to_string(), ":foo_bar:\n");

        // todo:: 添加中文的支持
        // let n3 = parse_from_string(":中文🐱:").unwrap();
        // assert_eq!(n3.to_string(), ":中文🐱:\n");
    }

    // primary expressions

    #[test]
    fn test_prefix_identifier() {
        let n1 = parse_from_string("!foo").unwrap();
        assert_eq!(n1.to_string(), "!foo\n");

        let n2 = parse_from_string("!foo::bar").unwrap();
        assert_eq!(n2.to_string(), "!foo::bar\n");
    }

    #[test]
    fn test_tuple() {
        let n1 = parse_from_string("(123,)").unwrap(); // 括号内的逗号不能省略
        assert_eq!(n1.to_string(), "(123,)\n");

        // 多个元素
        let n2 = parse_from_string("(123,1.732)").unwrap();
        assert_eq!(n2.to_string(), "(123, 1.732,)\n");

        // 元素列表以逗号结尾
        let n3 = parse_from_string("(123,1.732,)").unwrap();
        assert_eq!(n3.to_string(), "(123, 1.732,)\n");

        // 空元组
        let n4 = parse_from_string("()").unwrap();
        assert_eq!(n4.to_string(), "()\n");

        // 带有省略号元素的元组
        let n5 = parse_from_string("(123,...)").unwrap();
        assert_eq!(n5.to_string(), "(123, ...,)\n");

        // 带有省略号标识符元素的元组
        let n6 = parse_from_string("(123,...abc)").unwrap();
        assert_eq!(n6.to_string(), "(123, ...abc,)\n");

        // 逗号结尾
        let n7 = parse_from_string("(123,...abc,)").unwrap();
        assert_eq!(n7.to_string(), "(123, ...abc,)\n");

        // 多行格式
        let n8 = parse_from_string(&trim_left_margin(
            "(
                123,
                456,
                789
            )",
        ))
        .unwrap();
        assert_eq!(n8.to_string(), "(123, 456, 789,)\n");
    }

    #[test]
    fn test_list() {
        let n1 = parse_from_string("[123]").unwrap();
        assert_eq!(n1.to_string(), "[123,]\n");

        // 元素列表以 `逗号` 结尾
        let n2 = parse_from_string("[123,]").unwrap();
        assert_eq!(n2.to_string(), "[123,]\n");

        // 多个元素
        let n3 = parse_from_string("[123,1.732]").unwrap();
        assert_eq!(n3.to_string(), "[123, 1.732,]\n");

        // 元素列表以逗号结尾
        let n4 = parse_from_string("[123,1.732,]").unwrap();
        assert_eq!(n4.to_string(), "[123, 1.732,]\n");

        // 空列表
        let n5 = parse_from_string("[]").unwrap();
        assert_eq!(n5.to_string(), "[]\n");

        // 带有省略号元素的列表
        let n6 = parse_from_string("[123,...]").unwrap();
        assert_eq!(n6.to_string(), "[123, ...,]\n");

        // 带有省略号标识符元素的列表
        let n7 = parse_from_string("[123,...abc]").unwrap();
        assert_eq!(n7.to_string(), "[123, ...abc,]\n");

        // 逗号结尾
        let n8 = parse_from_string("[123,...abc,]").unwrap();
        assert_eq!(n8.to_string(), "[123, ...abc,]\n");

        // 范围表达式的列表
        let n9 = parse_from_string("[1..10]").unwrap();
        assert_eq!(n9.to_string(), "[1..10,]\n");

        // 逗号结尾
        let n10 = parse_from_string("[1..10,]").unwrap();
        assert_eq!(n10.to_string(), "[1..10,]\n");

        // "省略了范围结束值的范围表达式" 的列表
        let n11 = parse_from_string("[1..]").unwrap();
        assert_eq!(n11.to_string(), "[1..,]\n");

        // 逗号结尾
        let n12 = parse_from_string("[1..,]").unwrap();
        assert_eq!(n12.to_string(), "[1..,]\n");

        // 一个元素，以及一个范围表达式的列表
        let n13 = parse_from_string("[1,3..10]").unwrap();
        assert_eq!(n13.to_string(), "[1, 3..10,]\n");

        // 一个元素，以及一个省略了结束值的范围表达式的列表
        let n14 = parse_from_string("[1,3..]").unwrap();
        assert_eq!(n14.to_string(), "[1, 3..,]\n");

        // 闭区间
        let n15 = parse_from_string("[1..=10]").unwrap();
        assert_eq!(n15.to_string(), "[1..=10,]\n");

        // 一个元素，以及一个闭区间范围表达式的列表
        let n16 = parse_from_string("[1,3..=9]").unwrap();
        assert_eq!(n16.to_string(), "[1, 3..=9,]\n");

        // 多行格式
        let n17 = parse_from_string(&trim_left_margin(
            "[
                123,
                456,
                789
            ]",
        ))
        .unwrap();
        assert_eq!(n17.to_string(), "[123, 456, 789,]\n");
    }

    #[test]
    fn test_map() {
        let n1 = parse_from_string("{name:\"foo\"}").unwrap();
        assert_eq!(
            n1.to_string(),
            trim_left_margin(
                "{
                    name: \"foo\"
                }
                "
            )
        );

        let n2 = parse_from_string("{x:10,y:20}").unwrap();
        assert_eq!(
            n2.to_string(),
            trim_left_margin(
                "{
                    x: 10
                    y: 20
                }
                "
            )
        );

        // 以逗号结尾
        let n3 = parse_from_string("{x:10,y:20,}").unwrap();
        assert_eq!(
            n3.to_string(),
            trim_left_margin(
                "{
                    x: 10
                    y: 20
                }
                "
            )
        );

        // 多行格式
        let n3 = parse_from_string(&trim_left_margin(
            "{
                x:10
                y:20
                z:30
            }",
        ))
        .unwrap();
        assert_eq!(
            n3.to_string(),
            trim_left_margin(
                "{
                    x: 10
                    y: 20
                    z: 30
                }
                "
            )
        );

        // 多行格式带逗号
        let n4 = parse_from_string(&trim_left_margin(
            "{
                x:10,
                y:20,
                z:30,
            }",
        ))
        .unwrap();
        assert_eq!(
            n4.to_string(),
            trim_left_margin(
                "{
                    x: 10
                    y: 20
                    z: 30
                }
                "
            )
        );

        // 测试缺少 `value` 部分的
        let n5 = parse_from_string(&trim_left_margin(
            "{
                x
                y:20,
                z
            }",
        ))
        .unwrap();
        assert_eq!(
            n5.to_string(),
            trim_left_margin(
                "{
                    x
                    y: 20
                    z
                }
                "
            )
        );

        // 测试 `省略号表达式`
        let n6 = parse_from_string("{x, y:20, ...rest}").unwrap();
        assert_eq!(
            n6.to_string(),
            trim_left_margin(
                "{
                    x
                    y: 20
                    ...rest
                }
                "
            )
        );

        // 测试多行格式的 `省略号表达式`
        let n7 = parse_from_string(&trim_left_margin(
            "{
                x
                y:20,
                ...rest
            }",
        ))
        .unwrap();
        assert_eq!(
            n7.to_string(),
            trim_left_margin(
                "{
                    x
                    y: 20
                    ...rest
                }
                "
            )
        );
    }

    #[test]
    fn test_identifier() {
        let n1 = parse_from_string("foo").unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Identifier(Identifier {
                    dirs: vec![],
                    name: "foo".to_string(),
                    generics: vec![],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(n1.to_string(), "foo\n");

        // 测试名称空间路径
        let n2 = parse_from_string("foo::bar").unwrap();
        assert_eq!(n2.to_string(), "foo::bar\n");

        let n3 = parse_from_string("foo::bar::baz").unwrap();
        assert_eq!(n3.to_string(), "foo::bar::baz\n");

        // 测试泛型
        let n4 = parse_from_string("Point<Int>").unwrap();
        assert_eq!(n4.to_string(), "Point<Int>\n");

        // 测试多类型泛型
        let n5 = parse_from_string("Result<T,E>").unwrap();
        assert_eq!(n5.to_string(), "Result<T, E>\n");

        // 测试嵌套泛型
        let n6 = parse_from_string("Option<List<T>>").unwrap();
        assert_eq!(n6.to_string(), "Option<List<T>>\n");

        // 测试名称空间路径+泛型
        let n7 = parse_from_string("std::Result<T,E>").unwrap();
        assert_eq!(n7.to_string(), "std::Result<T, E>\n");
    }

    #[test]
    fn test_sign() {
        let n1 = parse_from_string("sign(Int a,Boolean b)type String").unwrap();
        assert_eq!(n1.to_string(), "sign (Int a, Boolean b) type String\n");

        // 无返回类型
        let n2 = parse_from_string("sign(Int a,Boolean b)").unwrap();
        assert_eq!(n2.to_string(), "sign (Int a, Boolean b)\n");

        // 无参数名称
        let n3 = parse_from_string("sign(Int,Boolean)type(Int,String)").unwrap();
        assert_eq!(n3.to_string(), "sign (Int, Boolean) type (Int, String,)\n");

        // 泛型
        let n4 = parse_from_string("sign<T>(T a, T b)type T").unwrap();
        assert_eq!(n4.to_string(), "sign <T> (T a, T b) type T\n");

        // 多类型泛型
        let n5 = parse_from_string("sign<T,E>(T, E)type E").unwrap();
        assert_eq!(n5.to_string(), "sign <T, E> (T, E) type E\n");

        // 测试单行 which
        let n6 = parse_from_string("sign(T a)type T which T:Int").unwrap();
        assert_eq!(
            n6.to_string(),
            trim_left_margin(
                "sign (T a) type T which {
                    T: Int
                }\n"
            )
        );

        // 测试单行 which + limit 单个数据类型代号
        let n7 = parse_from_string("sign (T a) which T:limit Display").unwrap();
        assert_eq!(
            n7.to_string(),
            trim_left_margin(
                "sign (T a) which {
                            T: limit Display
                        }\n"
            )
        );

        // 测试单行 which + limit 多个数据类型代号
        let n8 = parse_from_string("sign (T a) which T:limit Int+Display+Eq").unwrap();
        assert_eq!(
            n8.to_string(),
            trim_left_margin(
                "sign (T a) which {
                    T: limit Int + Display + Eq
                }\n"
            )
        );

        // 测试 which 表达式块
        let n9 = parse_from_string(&trim_left_margin(
            "sign (T t, E e, U u) which{
                T:Int
                E:limit Int+Display+Eq
                U:limit Eq
            }",
        ))
        .unwrap();
        assert_eq!(
            n9.to_string(),
            trim_left_margin(
                "sign (T t, E e, U u) which {
                    T: Int
                    E: limit Int + Display + Eq
                    U: limit Eq
                }\n"
            )
        );

        // 测试 which 表达式块，行末带逗号
        let n10 = parse_from_string(&trim_left_margin(
            "sign (T t, E e, U u) which{
                T:Int,
                E:limit Int+Display+Eq,
                U:limit Eq,
            }",
        ))
        .unwrap();
        assert_eq!(
            n10.to_string(),
            trim_left_margin(
                "sign (T t, E e, U u) which {
                    T: Int
                    E: limit Int + Display + Eq
                    U: limit Eq
                }\n"
            )
        );
    }

    #[test]
    fn test_anonymous_function() {
        let n1 = parse_from_string("fn (Int a, Boolean b) type String = 1+2").unwrap();
        assert_eq!(
            n1.to_string(),
            "fn (Int a, Boolean b) type String = (1 + 2)\n"
        );

        // 无返回类型
        let n2 = parse_from_string("fn (Int a, Boolean b) = 1+2*3").unwrap();
        assert_eq!(n2.to_string(), "fn (Int a, Boolean b) = (1 + (2 * 3))\n");

        // 无数据类型
        let n3 = parse_from_string("fn (a,b) = a+b").unwrap();
        assert_eq!(n3.to_string(), "fn (a, b) = (a + b)\n");

        // 单独一个参数
        let n4 = parse_from_string("fn (a) = a+1").unwrap();
        assert_eq!(n4.to_string(), "fn (a) = (a + 1)\n");

        // 单独一个参数且省略参数列表的括号
        let n5 = parse_from_string("fn a = a+1").unwrap();
        assert_eq!(n5.to_string(), "fn (a) = (a + 1)\n");

        // 函数体为表达式块
        let n5 = parse_from_string("fn a {a+1}").unwrap();
        assert_eq!(
            n5.to_string(),
            trim_left_margin(
                "fn (a) {
                            (a + 1)
                        }
                        "
            )
        );

        // 函数体为多行表达式块
        let n6 = parse_from_string("fn(a,b){a+b\na-b}").unwrap();
        assert_eq!(
            n6.to_string(),
            trim_left_margin(
                "fn (a, b) {
                            (a + b)
                            (a - b)
                        }
                        "
            )
        );

        // 测试单行 which
        let n7 = parse_from_string("fn (T a) which T:Int=1+2").unwrap();
        assert_eq!(
            n7.to_string(),
            trim_left_margin(
                "fn (T a) which {
                                    T: Int
                                } = (1 + 2)\n"
            )
        );

        // 测试单行 which + limit 单个数据类型代号
        let n8 = parse_from_string("fn (T a) which T:limit Display=1+2").unwrap();
        assert_eq!(
            n8.to_string(),
            trim_left_margin(
                "fn (T a) which {
                            T: limit Display
                        } = (1 + 2)\n"
            )
        );

        // 测试单行 which + limit 多个数据类型代号
        let n9 = parse_from_string("fn (T a) which T:limit Int+Display+Eq=1+2").unwrap();
        assert_eq!(
            n9.to_string(),
            trim_left_margin(
                "fn (T a) which {
                    T: limit Int + Display + Eq
                } = (1 + 2)\n"
            )
        );

        // 测试 which 表达式块
        let n10 = parse_from_string(&trim_left_margin(
            "fn (T t, E e, U u) which{
                T:Int
                E:limit Int+Display+Eq
                U:limit Eq
            }=1+2",
        ))
        .unwrap();
        assert_eq!(
            n10.to_string(),
            trim_left_margin(
                "fn (T t, E e, U u) which {
                    T: Int
                    E: limit Int + Display + Eq
                    U: limit Eq
                } = (1 + 2)\n"
            )
        );

        // 测试 which 表达式块，行末带逗号
        let n11 = parse_from_string(&trim_left_margin(
            "fn (T t, E e, U u) which{
                T:Int,
                E:limit Int + Display + Eq,
                U:limit Eq,
            }=1+2",
        ))
        .unwrap();
        assert_eq!(
            n11.to_string(),
            trim_left_margin(
                "fn (T t, E e, U u) which {
                    T: Int
                    E: limit Int + Display + Eq
                    U: limit Eq
                } = (1 + 2)\n"
            )
        );
    }

    // operating expressions

    #[test]
    fn test_constructor_expression() {
        let e1 = parse_from_string("User{id:123,name:\"foo\"}").unwrap();
        assert_eq!(
            e1.to_string(),
            trim_left_margin(
                "User {
                    id: 123
                    name: \"foo\"
                }
                "
            )
        );

        // 测试 key 为 hash string
        let e2 = parse_from_string("User{#id:123,#name:\"foo\"}").unwrap();
        assert_eq!(
            e2.to_string(),
            trim_left_margin(
                "User {
                    #id: 123
                    #name: \"foo\"
                }
                "
            )
        );

        // 测试仅列出 key 名称
        let e3 = parse_from_string("User{id,name}").unwrap();
        assert_eq!(
            e3.to_string(),
            trim_left_margin(
                "User {
                    id
                    name
                }
                "
            )
        );

        // 测试 `省略号表达式`
        let e3 = parse_from_string("User{id,name,...user001}").unwrap();
        assert_eq!(
            e3.to_string(),
            trim_left_margin(
                "User {
                    id
                    name
                    ...user001
                }
                "
            )
        );
    }

    #[test]
    fn test_member_expression() {
        let n1 = parse_from_string("user.name").unwrap();
        assert_eq!(n1.to_string(), "(user.name)\n");

        // 数字属性
        let n2 = parse_from_string("user.0").unwrap();
        assert_eq!(n2.to_string(), "(user.0)\n");

        // 连续属性
        let n3 = parse_from_string("user.name.first").unwrap();
        assert_eq!(n3.to_string(), "((user.name).first)\n");

        // 字符串索引
        let n4 = parse_from_string("user[\"name\"]").unwrap();
        assert_eq!(n4.to_string(), "(user[\"name\"])\n");

        // 属性和索引混合
        let n5 = parse_from_string("users[0].name").unwrap();
        assert_eq!(n5.to_string(), "((users[0]).name)\n");
    }

    #[test]
    fn test_slice_expression() {
        let n1 = parse_from_string("users[0]").unwrap();
        assert_eq!(n1.to_string(), "(users[0])\n");

        let n2 = parse_from_string("users[123]").unwrap();
        assert_eq!(n2.to_string(), "(users[123])\n");

        // 切片
        let n3 = parse_from_string("users[0..10]").unwrap();
        assert_eq!(n3.to_string(), "(users[0..10])\n");

        // 闭区间切片
        let n4 = parse_from_string("users[0..=9]").unwrap();
        assert_eq!(n4.to_string(), "(users[0..=9])\n");

        // 索引为表达式
        let n5 = parse_from_string("users[ids[0]]").unwrap();
        assert_eq!(n5.to_string(), "(users[(ids[0])])\n");

        // 连续索引
        let n6 = parse_from_string("users[0][1]").unwrap();
        assert_eq!(n6.to_string(), "((users[0])[1])\n");
    }

    #[test]
    fn test_function_call_expression() {
        let n1 = parse_from_string("foo(1)").unwrap();
        assert_eq!(n1.to_string(), "(foo)(1)\n");

        // 多个参数
        let n2 = parse_from_string("foo(1,2)").unwrap();
        assert_eq!(n2.to_string(), "(foo)(1, 2)\n");

        // 多个参数，末尾有逗号
        let n3 = parse_from_string("foo(1,2,3,)").unwrap();
        assert_eq!(n3.to_string(), "(foo)(1, 2, 3)\n");

        // 参数换行
        let n4 = parse_from_string(&trim_left_margin(
            "foo(
                        1,
                        2,
                        3,
                    )",
        ))
        .unwrap();
        assert_eq!(n4.to_string(), "(foo)(1, 2, 3)\n");

        // 参数为表达式
        let n5 = parse_from_string(&trim_left_margin(
            "foo(
                1+1,
                true&&false,
                bar(3),
            )",
        ))
        .unwrap();
        assert_eq!(
            n5.to_string(),
            "(foo)((1 + 1), (true && false), (bar)(3))\n"
        );

        // 带名称的参数
        let n6 = parse_from_string("foo(id=1,count=2)").unwrap();
        assert_eq!(n6.to_string(), "(foo)(id=1, count=2)\n");

        // 位置参数和命名参数混合
        let n7 = parse_from_string("foo(id=1,count=(2+3))").unwrap();
        assert_eq!(n7.to_string(), "(foo)(id=1, count=(2 + 3))\n");

        // 位置参数和命名参数混合
        let n8 = parse_from_string("foo(1,count=2)").unwrap();
        assert_eq!(n8.to_string(), "(foo)(1, count=2)\n");

        // 连续调用
        let n9 = parse_from_string("foo(1)(2)").unwrap();
        assert_eq!(n9.to_string(), "((foo)(1))(2)\n");

        // 被调用者为属性
        let n10 = parse_from_string("foo.bar(1)").unwrap();
        assert_eq!(n10.to_string(), "((foo.bar))(1)\n");

        // 被调用者为索引
        let n11 = parse_from_string("foo[1](2)").unwrap();
        assert_eq!(n11.to_string(), "((foo[1]))(2)\n");

        // 被调用者为索引
        let n12 = parse_from_string("(foo & bar)(1)").unwrap();
        assert_eq!(n12.to_string(), "((foo & bar))(1)\n");
    }

    #[test]
    fn test_unary_expression() {
        //
    }

    #[test]
    fn test_binary_expression_additive() {
        let n1 = parse_from_string("1+2").unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::BinaryExpression(
                    BinaryExpression {
                        operator: Token::Plus,
                        left: Box::new(Expression::Literal(Literal::Integer(Integer {
                            value: 1,
                            range: new_range()
                        }))),
                        right: Box::new(Expression::Literal(Literal::Integer(Integer {
                            value: 2,
                            range: new_range()
                        }))),
                        range: new_range()
                    }
                ))],
                range: new_range()
            })
        );
        assert_eq!(n1.to_string(), "(1 + 2)\n"); // Statement 以符号 '\n' 结尾

        let n2 = parse_from_string("1+2+3").unwrap();
        assert_eq!(n2.to_string(), "((1 + 2) + 3)\n");

        let n3 = parse_from_string("1.414+1.732").unwrap();
        assert_eq!(n3.to_string(), "(1.414 + 1.732)\n");

        // 测试复数和加法并存的情况
        let n4 = parse_from_string("3+4i+9i").unwrap();
        assert_eq!(n4.to_string(), "(3+4i + 0+9i)\n");
    }

    #[test]
    fn test_binary_expression_precedence() {
        let n1 = parse_from_string("1|2||3").unwrap();
        assert_eq!(n1.to_string(), "(1 | (2 || 3))\n");

        let n2 = parse_from_string("1||2&&3").unwrap();
        assert_eq!(n2.to_string(), "(1 || (2 && 3))\n");

        let n3 = parse_from_string("1&&2==3").unwrap();
        assert_eq!(n3.to_string(), "(1 && (2 == 3))\n");

        let n4 = parse_from_string("1==2>3").unwrap();
        assert_eq!(n4.to_string(), "(1 == (2 > 3))\n");

        let n5 = parse_from_string("1>2:bit_or:3").unwrap();
        assert_eq!(n5.to_string(), "(1 > (2 :bit_or: 3))\n");

        let n6 = parse_from_string("1:bit_and:2++3").unwrap();
        assert_eq!(n6.to_string(), "(1 :bit_and: (2 ++ 3))\n");

        let n7 = parse_from_string("1++2+3").unwrap();
        assert_eq!(n7.to_string(), "(1 ++ (2 + 3))\n");

        let n8 = parse_from_string("1+2*3").unwrap();
        assert_eq!(n8.to_string(), "(1 + (2 * 3))\n");

        let n9 = parse_from_string("1*2??3").unwrap();
        assert_eq!(n9.to_string(), "(1 * (2 ?? 3))\n");

        let n10 = parse_from_string("1??2->3").unwrap();
        assert_eq!(n10.to_string(), "(1 ?? (2 -> 3))\n");

        let n11 = parse_from_string("1->2&3").unwrap();
        assert_eq!(n11.to_string(), "(1 -> (2 & 3))\n");
    }

    #[test]
    fn test_binary_expression_parenthesized() {
        let n1 = parse_from_string("(123)").unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Literal(
                    Literal::Integer(Integer {
                        value: 123,
                        range: new_range()
                    })
                ))],
                range: new_range()
            })
        );
        assert_eq!(n1.to_string(), "123\n");

        let n2 = parse_from_string("(1+2)").unwrap();
        assert_eq!(n2.to_string(), "(1 + 2)\n");

        let n3 = parse_from_string("(1+2)*3").unwrap();
        assert_eq!(n3.to_string(), "((1 + 2) * 3)\n");
    }

    #[test]
    fn test_binary_expression_associativitye() {
        // 测试结合方向

        // 操作符 `+` 从左向右结合
        let n1 = parse_from_string("1+2+3").unwrap();
        assert_eq!(n1.to_string(), "((1 + 2) + 3)\n");

        // 操作符 `&` 从右向左结合
        let n2 = parse_from_string("1&2&3").unwrap();
        assert_eq!(n2.to_string(), "(1 & (2 & 3))\n");
    }

    // genernal expression

    #[test]
    fn test_do_expression() {
        let n1 = parse_from_string(
            "do {
                123
                abc
            }",
        )
        .unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::BlockExpression(
                    BlockExpression {
                        is_explicit: true,
                        body: vec![
                            Expression::Literal(Literal::Integer(Integer {
                                value: 123,
                                range: new_range()
                            })),
                            Expression::Identifier(Identifier {
                                dirs: vec![],
                                name: "abc".to_string(),
                                generics: vec![],
                                range: new_range()
                            }),
                        ],
                        range: new_range()
                    }
                ))],
                range: new_range()
            })
        );

        assert_eq!(
            n1.to_string(),
            trim_left_margin(
                "do {
                    123
                    abc
                }\n"
            )
        );
    }

    #[test]
    fn test_let_expression() {
        let n1 = parse_from_string("let Int i=1").unwrap();
        assert_eq!(n1.to_string(), "let Int i = 1\n");

        // 省略数据类型
        let n2 = parse_from_string("let i=100").unwrap();
        assert_eq!(n2.to_string(), "let i = 100\n");

        // 右手边值是一个表达式
        let n3 = parse_from_string("let i=1+2*3").unwrap();
        assert_eq!(n3.to_string(), "let i = (1 + (2 * 3))\n");

        // 左手边值是一个元组
        let n4 = parse_from_string("let (Int,Int) (x,y)=foo.point").unwrap();
        assert_eq!(n4.to_string(), "let (Int, Int,) (x, y,) = (foo.point)\n");

        // 左手边值是一个元组，省略数据类型
        let n5 = parse_from_string("let (x,y)=foo.point").unwrap();
        assert_eq!(n5.to_string(), "let (x, y,) = (foo.point)\n");

        // 连续赋值
        let n6 = parse_from_string("let a=let b=1").unwrap();
        assert_eq!(
            n6,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::LetExpression(
                    LetExpression {
                        data_type: None,
                        object: Box::new(Expression::Identifier(new_identifier("a"))),
                        value: Box::new(Expression::LetExpression(LetExpression {
                            data_type: None,
                            object: Box::new(Expression::Identifier(new_identifier("b"))),
                            value: Box::new(Expression::Literal(new_literal_integer(1))),
                            range: new_range()
                        })),
                        range: new_range()
                    }
                ))],
                range: new_range()
            })
        );
        assert_eq!(n6.to_string(), "let a = let b = 1\n");
    }

    #[test]
    fn test_join_expression() {
        let n1 = parse_from_string(
            "join {
                    123
                    abc
                }",
        )
        .unwrap();
        assert_eq!(
            n1.to_string(),
            trim_left_margin(
                "join {
                    123
                    abc
                }\n"
            )
        );
    }

    #[test]
    fn test_if_expression() {
        // todo::
    }

    #[test]
    fn test_for_expression() {
        // todo::
    }

    #[test]
    fn test_each_expression() {
        // todo::
    }

    #[test]
    fn test_branch_expression() {
        // todo::
    }

    #[test]
    fn test_match_expression() {
        // todo::
    }

    // statements

    #[test]
    fn test_function_declaration_statement() {
        // todo::
    }
}
