use anyhow::{bail, Context, Ok, Result};
use compact_str::CompactString;
use getset::Getters;

use crate::tokenizer::{Keywords, Line, Symbols, Token, Tokens};

pub type ClassName = CompactString;
#[derive(Clone)]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(ClassName),
}

#[derive(Clone)]
pub enum RetType {
    Void,
    Int,
    Char,
    Boolean,
    ClassName(ClassName),
}

pub enum UnaryOp {
    Negation,
    Not,
}

type ExpressionList = Vec<Expression>;
pub enum SubroutineCall {
    Direct(SubRoutineName, ExpressionList),
    Indirect(VarName, SubRoutineName, ExpressionList),
}

pub enum Term {
    Array(VarName, Box<Expression>),
    Parenthesis(Box<Expression>),
    Unary(UnaryOp, Box<Term>),
    SubRoutineCall(SubroutineCall),
    IntegerLiteral(u16),
    StringLiteral(CompactString),
    Variable(VarName),
    True,
    False,
    Null,
    This,
}

pub enum BinaryOp {
    Add,
    Minus,
    Mult,
    Div,
    And,
    Or,
    Lt,
    Gt,
    Eq,
}

#[derive(Getters)]
#[get = "pub"]
pub struct Expression {
    ops: Vec<BinaryOp>,
    terms: Vec<Term>,
}

pub enum LetStatement {
    Let(VarName, Expression),
    LetArray(VarName, Expression, Expression),
}

pub enum IfStatement {
    If(Expression, Vec<Statement>),
    IfElse(Expression, Vec<Statement>, Vec<Statement>),
}

#[derive(Getters)]
#[get = "pub"]
pub struct DoStatement {
    call: SubroutineCall,
}

#[derive(Getters)]
#[get = "pub"]
pub struct WhileStatement {
    condition: Expression,
    statements: Vec<Statement>,
}

pub enum ReturnStatement {
    Return,
    ReturnExpression(Expression),
}

pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    Do(DoStatement),
    While(WhileStatement),
    Return(ReturnStatement),
}

type VarName = CompactString;

#[derive(Getters)]
#[get = "pub"]
pub struct VarDeclaration {
    var_type: Type,
    var_name: VarName,
}

#[derive(Getters)]
#[get = "pub"]
pub struct SubRoutineBody {
    var_declarations: Vec<VarDeclaration>,
    statements: Vec<Statement>,
}

pub type SubRoutineName = CompactString;
pub type ParameterLists = Vec<VarDeclaration>;
pub enum SubRoutine {
    Constructor(RetType, SubRoutineName, ParameterLists, SubRoutineBody),
    Function(RetType, SubRoutineName, ParameterLists, SubRoutineBody),
    Method(RetType, SubRoutineName, ParameterLists, SubRoutineBody),
}

pub enum ClassVarDeclaration {
    Static(VarDeclaration),
    Field(VarDeclaration),
}

#[derive(Getters)]
#[get = "pub"]
pub struct ParseTree {
    name: ClassName,
    // Next time when impl ParseTree
    // 1. Preserve the source information for the decls and statements: (line, col, start_index, end_index)
    // as this can help us provide useful error messages
    var_declarations: Vec<ClassVarDeclaration>,
    subroutines: Vec<SubRoutine>,
}

struct TokenConsumer {
    tokens: Tokens,
    line: usize,
}

impl TokenConsumer {
    fn new(tokens: Tokens) -> TokenConsumer {
        TokenConsumer { tokens, line: 1 }
    }

    pub fn pop(&mut self) -> Option<&Token> {
        if let Some(Line { inner, line }) = self.tokens.pop() {
            self.line = *line;
            return Some(inner);
        }
        None
    }

    pub fn peek(&self, ith: usize) -> Option<&Token> {
        self.tokens.peek(ith).map(|t| &t.inner)
    }

    pub fn consume<F>(&mut self, f: F, err: &str) -> Result<()>
    where
        F: FnOnce(&Token) -> bool,
    {
        if self.pop().is_some_and(f) {
            return Ok(());
        }
        bail!("Line {}: {}", self.line, err);
    }

    pub fn report<E>(&mut self, err: &str) -> Result<E> {
        bail!("Line {}: {}", self.line, err);
    }
}

pub fn parse(tokens: Tokens) -> Result<ParseTree> {
    let mut consumer = TokenConsumer::new(tokens);
    parse_class(&mut consumer)
}

fn parse_class(consumer: &mut TokenConsumer) -> Result<ParseTree> {
    consumer.consume(
        |token| matches!(token, Token::Keywords(keywords) if matches!(keywords, Keywords::Class)),
        "Expect `class` keyword",
    )?;

    let class_name = if let Some(Token::Identifier(identifier)) = consumer.pop() {
        identifier.clone()
    } else {
        return consumer.report("Expect class name");
    };

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftCurly)),
        "Expect `{`",
    )?;

    let class_vars = {
        let mut class_vars: Vec<ClassVarDeclaration> = vec![];
        while let Some(Token::Keywords(keyword)) = consumer.peek(0) {
            if !matches!(keyword, Keywords::Static | Keywords::Field) {
                break;
            }
            class_vars.extend(
                parse_class_var_declare(consumer).context("Failed to parse class variables")?,
            );
        }
        class_vars
    };

    let subroutines = {
        let mut subroutines: Vec<SubRoutine> = vec![];
        while let Some(Token::Keywords(keyword)) = consumer.peek(0) {
            if !matches!(
                keyword,
                Keywords::Constructor | Keywords::Function | Keywords::Method
            ) {
                break;
            }
            subroutines
                .push(parse_subroutine(consumer).context("Failed to parse class subroutine")?);
        }
        subroutines
    };

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightCurly)),
        "Expect `}`",
    )?;

    Ok(ParseTree {
        name: class_name,
        var_declarations: class_vars,
        subroutines,
    })
}

fn parse_var_declare(consumer: &mut TokenConsumer) -> Result<Vec<VarDeclaration>> {
    let var_type = parse_type(consumer).context("Failed to parse variable type")?;
    let mut vars = vec![];

    // process variable names
    loop {
        let var_name = if let Some(Token::Identifier(identifier)) = consumer.pop() {
            identifier.clone()
        } else {
            return consumer.report("Expect class variable name");
        };
        vars.push(VarDeclaration {
            var_type: var_type.clone(),
            var_name,
        });

        if consumer
            .peek(0)
            .is_some_and(|token| matches!(token, Token::Symbols(s) if matches!(s, Symbols::Comma)))
        {
            consumer.pop();
            continue;
        }
        break;
    }

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::SemiColon)),
        "Expect `;`",
    )?;

    Ok(vars)
}

// Precondition: first token == `static` or `field`
fn parse_class_var_declare(consumer: &mut TokenConsumer) -> Result<Vec<ClassVarDeclaration>> {
    let keyword = if let Some(Token::Keywords(keyword)) = consumer.pop() {
        *keyword
    } else {
        unreachable!();
    };

    let vars = parse_var_declare(consumer).context("Failed to parse class variable declaration")?;

    Ok(match keyword {
        Keywords::Static => vars.into_iter().map(ClassVarDeclaration::Static).collect(),
        Keywords::Field => vars.into_iter().map(ClassVarDeclaration::Field).collect(),
        _ => unreachable!(),
    })
}

fn parse_type(consumer: &mut TokenConsumer) -> Result<Type> {
    if let Some(token) = consumer.pop() {
        return match token {
            Token::Keywords(keyword) => match keyword {
                Keywords::Int => Ok(Type::Int),
                Keywords::Char => Ok(Type::Char),
                Keywords::Boolean => Ok(Type::Boolean),
                _ => consumer.report("Expect `int`, `char`, or `boolean`"),
            },
            Token::Identifier(classname) => Ok(Type::ClassName(classname.clone())),
            _ => consumer.report("Expect `int`, `char`, `boolean`, or class name"),
        };
    }
    consumer.report("Expect `int`, `char`, `boolean`, or class name")
}

fn parse_func_return_type(consumer: &mut TokenConsumer) -> Result<RetType> {
    if let Some(token) = consumer.pop() {
        return match token {
            Token::Keywords(keyword) => match keyword {
                Keywords::Int => Ok(RetType::Int),
                Keywords::Char => Ok(RetType::Char),
                Keywords::Boolean => Ok(RetType::Boolean),
                Keywords::Void => Ok(RetType::Void),
                _ => consumer.report("Expect `void`, `int`, `char`, or `boolean`"),
            },
            Token::Identifier(classname) => Ok(RetType::ClassName(classname.clone())),
            _ => consumer.report("Expect `void`, `int`, `char`, `boolean`, or class name"),
        };
    }
    consumer.report("Expect `void`, int`, `char`, `boolean`, or class name")
}

// Precondition: first token == `constructor`, `function`, or `method`
fn parse_subroutine(consumer: &mut TokenConsumer) -> Result<SubRoutine> {
    let subroutine_type = if let Some(Token::Keywords(keyword)) = consumer.pop() {
        match keyword {
            Keywords::Constructor | Keywords::Function | Keywords::Method => *keyword,
            _ => unreachable!(),
        }
    } else {
        return consumer.report("Expect `constructor`, `function`, or `method` keyword");
    };
    let ret_type =
        parse_func_return_type(consumer).context("Failed to parse subroutine return type")?;
    let subroutine_name = if let Some(Token::Identifier(identifier)) = consumer.pop() {
        identifier.clone()
    } else {
        return consumer.report("Expect subroutine name");
    };

    let parameters =
        parse_parameter_list(consumer).context("Failed to parse subroutine parameters")?;
    let body = parse_subroutine_body(consumer).context("Failed to parse subroutine body")?;

    Ok(match subroutine_type {
        Keywords::Constructor => {
            SubRoutine::Constructor(ret_type, subroutine_name, parameters, body)
        }
        Keywords::Function => SubRoutine::Function(ret_type, subroutine_name, parameters, body),
        Keywords::Method => SubRoutine::Method(ret_type, subroutine_name, parameters, body),
        _ => unreachable!(),
    })
}

fn parse_subroutine_body(consumer: &mut TokenConsumer) -> Result<SubRoutineBody> {
    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftCurly)),
        "Expect `{`",
    )?;

    let mut vars = vec![];
    while let Some(Token::Keywords(keyword)) = consumer.peek(0) {
        if !matches!(keyword, Keywords::Var) {
            break;
        }
        vars.extend(parse_subroutine_var_declare(consumer)?);
    }

    let statements = parse_statements(consumer).context("Failed to parse subroutine statements")?;

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightCurly)),
        "Expect `}`",
    )?;

    Ok(SubRoutineBody {
        var_declarations: vars,
        statements,
    })
}

fn parse_subroutine_call(consumer: &mut TokenConsumer) -> Result<SubroutineCall> {
    let subroutine_name_or_class_var_name =
        if let Some(Token::Identifier(identifier)) = consumer.pop() {
            identifier.clone()
        } else {
            return consumer.report("Expect subroutine, class, or variable name");
        };

    fn parse_arguments(consumer: &mut TokenConsumer) -> Result<ExpressionList> {
        let arguments = parse_expression_list(consumer)
            .context("Failed to parse arguments for subroutine call")?;
        Ok(arguments)
    }

    if let Some(Token::Symbols(symbols)) = consumer.peek(0) {
        match symbols {
            Symbols::Dot => {
                // already checked that this is a dot
                consumer.pop();

                let subroutine_name = if let Some(Token::Identifier(identifier)) = consumer.pop() {
                    identifier.clone()
                } else {
                    return consumer.report("Expect subroutine name");
                };
                Ok(SubroutineCall::Indirect(
                    subroutine_name_or_class_var_name,
                    subroutine_name,
                    parse_arguments(consumer)?,
                ))
            }
            Symbols::LeftParen => Ok(SubroutineCall::Direct(
                subroutine_name_or_class_var_name,
                parse_arguments(consumer)?,
            )),
            _ => consumer.report("Expect `.` or `(`"),
        }
    } else {
        consumer.report("Expect `.` or `(`")
    }
}

// Diff: here we use '(' parameters ')' to represent parameter list
fn parse_parameter_list(consumer: &mut TokenConsumer) -> Result<ParameterLists> {
    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftParen)),
        "Expect `(`",
    )?;

    // empty list
    if consumer
        .peek(0)
        .is_some_and(|token| matches!(token, Token::Symbols(s) if matches!(s, Symbols::RightParen)))
    {
        consumer.pop();
        return Ok(vec![]);
    }

    let mut vars = vec![];
    loop {
        let var_type = parse_type(consumer)?;
        let var_name = if let Some(Token::Identifier(identifier)) = consumer.pop() {
            identifier.clone()
        } else {
            return consumer.report("Expect variable name");
        };
        vars.push(VarDeclaration { var_type, var_name });

        // if comma => continue; otherwise, always break
        if consumer
            .peek(0)
            .is_some_and(|token| matches!(token, Token::Symbols(x) if matches!(x, Symbols::Comma)))
        {
            consumer.pop();
            continue;
        }

        break;
    }

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightParen)),
        "Expect `)`",
    )?;

    Ok(vars)
}

// Precondition: know the first token is var
fn parse_subroutine_var_declare(consumer: &mut TokenConsumer) -> Result<Vec<VarDeclaration>> {
    consumer.pop();

    Ok(parse_var_declare(consumer).context("Failed to parse subroutine variable declaration")?)
}

fn parse_statements(consumer: &mut TokenConsumer) -> Result<Vec<Statement>> {
    let mut statement = vec![];

    while let Some(token) = consumer.peek(0) {
        match token {
            Token::Keywords(keyword) => match keyword {
                Keywords::Let => {
                    statement.push(Statement::Let(
                        parse_let(consumer).context("Failed to parse `let` statement")?,
                    ));
                }
                Keywords::If => {
                    statement.push(Statement::If(
                        parse_if(consumer).context("Failed to parse `if` statement")?,
                    ));
                }
                Keywords::While => {
                    statement.push(Statement::While(
                        parse_while(consumer).context("Failed to parse `while` statement")?,
                    ));
                }
                Keywords::Do => {
                    statement.push(Statement::Do(
                        parse_do(consumer).context("Failed to parse `do` statement")?,
                    ));
                }
                Keywords::Return => {
                    statement.push(Statement::Return(
                        parse_return(consumer).context("Failed to parse `return` statement")?,
                    ));
                }
                _ => break,
            },
            _ => break,
        }
    }

    Ok(statement)
}

// Precondition: know the first token is let
fn parse_let(consumer: &mut TokenConsumer) -> Result<LetStatement> {
    consumer.pop();

    let var_name = if let Some(Token::Identifier(identifier)) = consumer.pop() {
        identifier.clone()
    } else {
        return consumer.report("Expect variable name");
    };

    fn parse_assignment(consumer: &mut TokenConsumer) -> Result<Expression> {
        consumer.consume(
            |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::Equal)),
            "Expect `=`",
        )?;
        let expr = parse_expression(consumer);
        consumer.consume(
            |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::SemiColon)),
                "Expect `;`",
        )?;
        expr
    }

    if consumer
        .peek(0)
        .is_some_and(|token| matches!(token, Token::Symbols(s) if matches!(s, Symbols::LeftSquare)))
    {
        consumer.pop();
        let expr = parse_expression(consumer).context("Failed to array index expression")?;
        consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightSquare)),
            "Expect `]`",
        )?;

        return Ok(LetStatement::LetArray(
            var_name,
            expr,
            parse_assignment(consumer)?,
        ));
    }

    Ok(LetStatement::Let(var_name, parse_assignment(consumer)?))
}

// Precondition: know the first token is if
fn parse_if(consumer: &mut TokenConsumer) -> Result<IfStatement> {
    consumer.pop();

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftParen)),
        "Expect `(`",
    )?;

    let expr = parse_expression(consumer).context("Failed to parse if condition")?;

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightParen)),
        "Expect `)`",
    )?;

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftCurly)),
        "Expect `{`",
    )?;

    let if_statements = parse_statements(consumer).context("Failed to parse if body")?;

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightCurly)),
        "Expect `}`",
    )?;

    if consumer
        .peek(0)
        .is_some_and(|token| matches!(token, Token::Keywords(k) if matches!(k, Keywords::Else)))
    {
        consumer.pop();
        consumer.consume(
            |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftCurly)),
            "Expect `{`",
        )?;

        let else_statements =
            parse_statements(consumer).context("Failed to parse else statements")?;

        consumer.consume(
            |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightCurly)),
            "Expect `}`",
        )?;

        return Ok(IfStatement::IfElse(expr, if_statements, else_statements));
    }

    Ok(IfStatement::If(expr, if_statements))
}

// Precondition: know the first token is while
fn parse_while(consumer: &mut TokenConsumer) -> Result<WhileStatement> {
    consumer.pop();

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftParen)),
        "Expect `(`",
    )?;

    let expr = parse_expression(consumer).context("Failed to parse if condition")?;

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightParen)),
        "Expect `)`",
    )?;

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftCurly)),
        "Expect `{`",
    )?;

    let while_statements = parse_statements(consumer).context("Failed to parse if statements")?;

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightCurly)),
        "Expect `}`",
    )?;

    Ok(WhileStatement {
        condition: expr,
        statements: while_statements,
    })
}

// Precondition: know the first token is do
fn parse_do(consumer: &mut TokenConsumer) -> Result<DoStatement> {
    consumer.pop();

    let call = parse_subroutine_call(consumer).context("Failed to parse subroutine call")?;

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::SemiColon)),
        "Expect `;`",
    )?;

    Ok(DoStatement { call })
}

// Precondition: know the first token is return
fn parse_return(consumer: &mut TokenConsumer) -> Result<ReturnStatement> {
    consumer.pop();

    if consumer
        .peek(0)
        .is_some_and(|token| matches!(token, Token::Symbols(s) if matches!(s, Symbols::SemiColon)))
    {
        consumer.pop();
        return Ok(ReturnStatement::Return);
    }

    let expr = parse_expression(consumer).context("Failed to parse return expression")?;
    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::SemiColon)),
        "Expect `;`",
    )?;
    Ok(ReturnStatement::ReturnExpression(expr))
}

fn parse_expression(consumer: &mut TokenConsumer) -> Result<Expression> {
    let mut ops = vec![];
    let mut terms = vec![parse_term(consumer).context("Failed to parse term")?];

    while let Some(token) = consumer.peek(0) {
        match token {
            Token::Symbols(symbol) => match symbol {
                Symbols::Plus => {
                    ops.push(BinaryOp::Add);
                }
                Symbols::Minus => {
                    ops.push(BinaryOp::Minus);
                }
                Symbols::Mult => {
                    ops.push(BinaryOp::Mult);
                }
                Symbols::Div => {
                    ops.push(BinaryOp::Div);
                }
                Symbols::And => {
                    ops.push(BinaryOp::And);
                }
                Symbols::Or => {
                    ops.push(BinaryOp::Or);
                }
                Symbols::Lt => {
                    ops.push(BinaryOp::Lt);
                }
                Symbols::Gt => {
                    ops.push(BinaryOp::Gt);
                }
                Symbols::Equal => {
                    ops.push(BinaryOp::Eq);
                }
                _ => break,
            },
            _ => break,
        }

        consumer.pop();

        terms.push(parse_term(consumer).context("Failed to parse term")?);
    }

    Ok(Expression { ops, terms })
}

fn parse_term(consumer: &mut TokenConsumer) -> Result<Term> {
    if let Some(token) = consumer.peek(0) {
        return match token {
            Token::Keywords(keyword) => match keyword {
                Keywords::True => {
                    consumer.pop();
                    return Ok(Term::True);
                }
                Keywords::False => {
                    consumer.pop();
                    return Ok(Term::False);
                }
                Keywords::Null => {
                    consumer.pop();
                    return Ok(Term::Null);
                }
                Keywords::This => {
                    consumer.pop();
                    return Ok(Term::This);
                }
                _ => consumer.report::<Term>("Expect a term"),
            },
            Token::Symbols(symbol) => match symbol {
                Symbols::Minus => {
                    consumer.pop();
                    let term = parse_term(consumer).context("Failed to parse negation")?;
                    return Ok(Term::Unary(UnaryOp::Negation, Box::new(term)));
                }
                Symbols::Not => {
                    consumer.pop();
                    let term = parse_term(consumer).context("Failed to parse not")?;
                    return Ok(Term::Unary(UnaryOp::Not, Box::new(term)));
                }
                Symbols::LeftParen => {
                    consumer.pop();
                    let expr = parse_expression(consumer).context("Failed to parse expression")?;
                    consumer.consume(
                        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightParen)),
                        "Expect `)`",
                    )?;
                    return Ok(Term::Parenthesis(Box::new(expr)));
                }
                _ => consumer.report::<Term>("Expect a term"),
            },
            Token::Integer(int) => {
                let int = *int;
                consumer.pop();
                return Ok(Term::IntegerLiteral(int));
            }
            Token::String(s) => {
                let s = s.clone();
                consumer.pop();
                return Ok(Term::StringLiteral(s));
            }
            Token::Identifier(name) => {
                if let Some(Token::Symbols(symbol)) = consumer.peek(1) {
                    match symbol {
                        Symbols::LeftSquare => {
                            let name = name.clone();
                            consumer.pop();

                            consumer.consume(
                                    |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftSquare)),
                                    "Expect `[`",
                                )?;
                            let expr = parse_expression(consumer)
                                .context("Failed to parse array index expression")?;
                            consumer.consume(
                                    |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightSquare)),
                                    "Expect `]`",
                                )?;

                            return Ok(Term::Array(name, Box::new(expr)));
                        }
                        Symbols::LeftParen | Symbols::Dot => {
                            return Ok(Term::SubRoutineCall(
                                parse_subroutine_call(consumer)
                                    .context("Failed to parse subroutine call")?,
                            ))
                        }
                        _ => (),
                    }
                }

                let name = name.clone();
                consumer.pop();
                return Ok(Term::Variable(name));
            }
        };
    }

    consumer.report("Expect a token")
}

// Diff: here we use '(' expressions ')' to represent parameter list
fn parse_expression_list(consumer: &mut TokenConsumer) -> Result<ExpressionList> {
    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::LeftParen)),
        "Expect `(`",
    )?;

    // empty list
    if consumer
        .peek(0)
        .is_some_and(|token| matches!(token, Token::Symbols(s) if matches!(s, Symbols::RightParen)))
    {
        consumer.pop();
        return Ok(vec![]);
    }

    let mut exprs = vec![];

    loop {
        let expr = parse_expression(consumer).context("Failed to parse expression")?;
        exprs.push(expr);

        if consumer
            .peek(0)
            .is_some_and(|token| matches!(token, Token::Symbols(x) if matches!(x, Symbols::Comma)))
        {
            consumer.pop();
            continue;
        }

        break;
    }

    consumer.consume(
        |token| matches!(token, Token::Symbols(symbols) if matches!(symbols, Symbols::RightParen)),
        "Expect `)`",
    )?;

    Ok(exprs)
}
