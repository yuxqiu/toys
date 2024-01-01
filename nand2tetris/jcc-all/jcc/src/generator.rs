use std::{iter::zip, vec};

use anyhow::{bail, Context, Ok, Result};
use compact_str::{format_compact, CompactString};
use delegate::delegate;
use getset::Getters;

use crate::{
    parser::{
        BinaryOp, ClassName, Expression, LetStatement, ParameterLists, ParseTree, Statement,
        SubRoutineBody, SubRoutineName, SubroutineCall, Term, VarDeclaration,
    },
    symbol_table::{ClassSymbolTable, LocalSymbolTable, SymbolTable, VarKind},
};

#[derive(Getters)]
#[get = "pub"]
struct FunctionContext<'a> {
    class_name: &'a ClassName,
    #[getset(skip)]
    symbols: SymbolTable<'a>,
    #[getset(skip)]
    counter: u32,
}

impl<'a> FunctionContext<'a> {
    pub fn new(class_name: &'a ClassName, symbols: SymbolTable<'a>) -> FunctionContext<'a> {
        FunctionContext {
            class_name,
            symbols,
            counter: 0,
        }
    }

    delegate! {
        to self.symbols {
            #[call(get)]
            pub fn get_symbol(&self, name: &str) -> Option<&(VarKind, u16)>;
        }
    }

    pub fn consume_counter(&mut self) -> u32 {
        let old = self.counter;
        self.counter += 1;
        old
    }
}

pub fn generate(tree: &ParseTree) -> Result<Vec<CompactString>> {
    let class_table = ClassSymbolTable::new(tree.var_declarations());

    let mut code = vec![];
    for subroutine in tree.subroutines() {
        let subroutine_code = match subroutine {
            crate::parser::SubRoutine::Constructor(_, name, params, body) => {
                generate_constructor(tree.name(), &class_table, name, params, body).with_context(
                    || {
                        format!(
                            "Failed to generate code for constructor {}.{}",
                            tree.name(),
                            name
                        )
                    },
                )?
            }
            crate::parser::SubRoutine::Function(_, name, params, body) => {
                generate_function(tree.name(), &class_table, name, params, body).with_context(
                    || {
                        format!(
                            "Failed to generate code for function {}.{}",
                            tree.name(),
                            name
                        )
                    },
                )?
            }
            crate::parser::SubRoutine::Method(_, name, params, body) => {
                generate_method(tree.name(), &class_table, name, params, body).with_context(
                    || {
                        format!(
                            "Failed to generate code for method {}.{}",
                            tree.name(),
                            name
                        )
                    },
                )?
            }
        };
        code.reserve(subroutine_code.len());
        code.extend(subroutine_code);
    }

    Ok(code)
}

fn generate_constructor(
    class_name: &ClassName,
    class_symbols: &ClassSymbolTable,
    func_name: &SubRoutineName,
    params: &ParameterLists,
    body: &SubRoutineBody,
) -> Result<Vec<CompactString>> {
    let class_len = class_symbols.len();
    let symbols = get_symbol_table(class_symbols, params, body.var_declarations(), None, false);
    let mut context = FunctionContext::new(class_name, symbols);

    let mut codes = vec![
        format_compact!(
            "function {}.{} {}",
            class_name,
            func_name,
            body.var_declarations().len()
        ),
        format_compact!("push constant {}", class_len),
        "call Memory.alloc 1".into(),
        "pop pointer 0".into(),
    ];

    let statement_code = generate_statements(&mut context, body.statements())?;
    codes.reserve_exact(statement_code.len());
    codes.extend(statement_code);

    Ok(codes)
}

fn generate_function(
    class_name: &ClassName,
    class_symbols: &ClassSymbolTable,
    func_name: &SubRoutineName,
    params: &ParameterLists,
    body: &SubRoutineBody,
) -> Result<Vec<CompactString>> {
    let symbols = get_symbol_table(class_symbols, params, body.var_declarations(), None, true);
    let mut context = FunctionContext::new(class_name, symbols);
    let mut codes = vec![format_compact!(
        "function {}.{} {}",
        class_name,
        func_name,
        body.var_declarations().len()
    )];

    let statement_code = generate_statements(&mut context, body.statements())?;
    codes.reserve_exact(statement_code.len());
    codes.extend(statement_code);

    Ok(codes)
}

fn generate_method(
    class_name: &ClassName,
    class_symbols: &ClassSymbolTable,
    func_name: &SubRoutineName,
    params: &ParameterLists,
    body: &SubRoutineBody,
) -> Result<Vec<CompactString>> {
    let symbols = get_symbol_table(
        class_symbols,
        params,
        body.var_declarations(),
        Some(class_name.clone()),
        false,
    );
    let mut context = FunctionContext::new(class_name, symbols);
    let mut codes = vec![
        format_compact!(
            "function {}.{} {}",
            class_name,
            func_name,
            body.var_declarations().len()
        ),
        "push argument 0".into(),
        "pop pointer 0".into(),
    ];

    let statement_code = generate_statements(&mut context, body.statements())?;
    codes.reserve_exact(statement_code.len());
    codes.extend(statement_code);

    Ok(codes)
}

fn generate_statements(
    context: &mut FunctionContext,
    statements: &Vec<Statement>,
) -> Result<Vec<CompactString>> {
    let mut code = vec![];

    for statement in statements {
        let statement_code: Vec<CompactString> = match statement {
            Statement::Let(let_statement) => match let_statement {
                LetStatement::Let(var_name, expr) => {
                    let var = context
                        .get_symbol(var_name)
                        .with_context(|| format!("Failed to find variable {var_name}"))?;

                    let mut expr_code = generate_expression(context, expr)?;
                    expr_code.reserve_exact(1);
                    expr_code.push(format_compact!(
                        "pop {} {}",
                        get_segment_name(&var.0),
                        var.1
                    ));

                    expr_code
                }
                LetStatement::LetArray(var_name, expr_lhs, expr_rhs) => {
                    let var = context
                        .get_symbol(var_name)
                        .with_context(|| format!("Failed to find variable {var_name}"))?;

                    let mut code = vec![format_compact!(
                        "push {} {}",
                        get_segment_name(&var.0),
                        var.1
                    )];

                    let expr_lhs_code = generate_expression(context, expr_lhs)?;
                    code.reserve(expr_lhs_code.len() + 1);
                    code.extend(expr_lhs_code);

                    // compute pointer for the left
                    code.push("add".into());

                    let expr_rhs_code = generate_expression(context, expr_rhs)?;
                    code.reserve(expr_rhs_code.len());
                    code.extend(expr_rhs_code);

                    // compute pointer for the right
                    code.reserve_exact(4);
                    code.push("pop temp 0".into());
                    code.push("pop pointer 1".into());
                    code.push("push temp 0".into());
                    code.push("pop that 0".into());

                    code
                }
            },
            Statement::If(if_statement) => match if_statement {
                crate::parser::IfStatement::If(expr, if_statements) => {
                    let if_end_counter = context.consume_counter();

                    let mut if_code = generate_expression(context, expr)?;

                    if_code.reserve(2);
                    if_code.push("not".into());
                    if_code.push(format_compact!("if-goto {}", if_end_counter));

                    let if_statements_code = generate_statements(context, if_statements)?;
                    if_code.reserve(if_statements_code.len());
                    if_code.extend(if_statements_code);

                    if_code.reserve_exact(1);
                    if_code.push(format_compact!("label {}", if_end_counter));

                    if_code
                }
                crate::parser::IfStatement::IfElse(expr, if_statements, else_statements) => {
                    let else_start_counter = context.consume_counter();
                    let else_end_counter = context.consume_counter();

                    let mut if_else_code = generate_expression(context, expr)?;

                    if_else_code.reserve(2);
                    if_else_code.push("not".into());
                    if_else_code.push(format_compact!("if-goto {}", else_start_counter));

                    let if_statements_code = generate_statements(context, if_statements)?;
                    if_else_code.reserve(if_statements_code.len());
                    if_else_code.extend(if_statements_code);

                    if_else_code.reserve(2);
                    if_else_code.push(format_compact!("goto {}", else_end_counter));
                    if_else_code.push(format_compact!("label {}", else_start_counter));

                    let else_statements_code = generate_statements(context, else_statements)?;
                    if_else_code.reserve(else_statements_code.len());
                    if_else_code.extend(else_statements_code);

                    if_else_code.reserve_exact(1);
                    if_else_code.push(format_compact!("label {}", else_end_counter));

                    if_else_code
                }
            },
            Statement::Do(do_statement) => {
                let mut code = generate_subroutine_call(context, do_statement.call())?;
                code.reserve_exact(1);
                code.push("pop temp 0".into());
                code
            }
            Statement::While(while_statement) => {
                let while_start_counter = context.consume_counter();
                let while_end_counter = context.consume_counter();

                let mut while_code = vec![format_compact!("label {}", while_start_counter)];

                let condition_code = generate_expression(context, while_statement.condition())?;
                while_code.reserve(condition_code.len());
                while_code.extend(condition_code);

                while_code.reserve(2);
                while_code.push("not".into());
                while_code.push(format_compact!("if-goto {}", while_end_counter));

                let statement_code = generate_statements(context, while_statement.statements())?;
                while_code.reserve(statement_code.len());
                while_code.extend(statement_code);

                while_code.reserve_exact(2);
                while_code.push(format_compact!("goto {}", while_start_counter));
                while_code.push(format_compact!("label {}", while_end_counter));

                while_code
            }
            Statement::Return(ret_statement) => match ret_statement {
                crate::parser::ReturnStatement::Return => {
                    vec!["push constant 0".into(), "return".into()]
                }
                crate::parser::ReturnStatement::ReturnExpression(expr) => {
                    let mut expr_code = generate_expression(context, expr)?;
                    expr_code.reserve_exact(1);
                    expr_code.push("return".into());
                    expr_code
                }
            },
        };

        code.reserve(statement_code.len());
        code.extend(statement_code);
    }

    Ok(code)
}

fn generate_expression(context: &FunctionContext, expr: &Expression) -> Result<Vec<CompactString>> {
    // safety: there is at least one term in expr
    let mut expr_code = generate_term(context, &expr.terms()[0])?;

    for (term, op) in zip(&expr.terms()[1..], expr.ops()) {
        let term_code = generate_term(context, term)?;
        expr_code.reserve(term_code.len() + 1);
        expr_code.extend(term_code);
        expr_code.push(get_binary_op_ins(op).into());
    }

    Ok(expr_code)
}

fn generate_term(context: &FunctionContext, term: &Term) -> Result<Vec<CompactString>> {
    Ok(match term {
        Term::Array(var_name, expr) => {
            let var = context
                .get_symbol(var_name)
                .with_context(|| format!("Failed to find variable {var_name}"))?;

            let mut array_code = vec![format_compact!(
                "push {} {}",
                get_segment_name(&var.0),
                var.1
            )];
            let expr_code = generate_expression(context, expr)?;
            array_code.reserve(expr_code.len());
            array_code.extend(expr_code);

            array_code.reserve_exact(3);
            array_code.push("add".into());
            array_code.push("pop pointer 1".into());
            array_code.push("push that 0".into());

            array_code
        }
        Term::Parenthesis(expr) => generate_expression(context, expr)?,
        Term::Unary(unary_op, term) => {
            let mut term_code = generate_term(context, term)?;
            term_code.reserve_exact(1);
            term_code.push(
                match unary_op {
                    crate::parser::UnaryOp::Negation => "neg",
                    crate::parser::UnaryOp::Not => "not",
                }
                .into(),
            );
            term_code
        }
        Term::SubRoutineCall(call) => generate_subroutine_call(context, call)?,
        Term::IntegerLiteral(int) => vec![format_compact!("push constant {}", int)],
        Term::StringLiteral(s) => {
            let mut string_code = vec![
                format_compact!("push constant {}", s.len()),
                "call String.new 1".into(),
            ];
            string_code.reserve(2 * s.len());

            for c in s.chars() {
                let c = c as u32;
                if c > 0xffff {
                    bail!("Failed to handle character `{}`: size > 16 bits", c);
                }
                string_code.push(format_compact!("push constant {}", c));
                string_code.push("call String.appendChar 2".into());
            }

            string_code
        }
        Term::Variable(var_name) => {
            let var = context
                .get_symbol(var_name)
                .with_context(|| format!("Failed to find variable {var_name}"))?;
            vec![format_compact!(
                "push {} {}",
                get_segment_name(&var.0),
                var.1
            )]
        }
        Term::True => vec!["push constant 1".into(), "neg".into()],
        Term::False | Term::Null => vec!["push constant 0".into()],
        Term::This => vec!["push pointer 0".into()],
    })
}

fn generate_subroutine_call(
    context: &FunctionContext,
    subroutine_call: &SubroutineCall,
) -> Result<Vec<CompactString>> {
    Ok(match subroutine_call {
        // this, by specification, is always a call to method in current class
        crate::parser::SubroutineCall::Direct(func_name, expr_list) => {
            let mut code = vec!["push pointer 0".into()];
            for expr in expr_list {
                let expr_code = generate_expression(context, expr)?;
                code.reserve(expr_code.len());
                code.extend(expr_code);
            }
            code.reserve_exact(1);
            code.push(format_compact!(
                "call {}.{} {}",
                context.class_name(),
                func_name,
                1 + expr_list.len()
            ));
            code
        }
        // if `name` can be found as a variable, we will treat it as a `var_name`
        // otherwise, it will be treated as a `class_name`
        crate::parser::SubroutineCall::Indirect(name, func_name, expr_list) => {
            let mut code = vec![];
            let potential_var = context.get_symbol(name);

            if let Some(var) = potential_var {
                code.push(format_compact!(
                    "push {} {}",
                    get_segment_name(&var.0),
                    var.1
                ));
            }

            for expr in expr_list {
                let expr_code = generate_expression(context, expr)?;
                code.reserve(expr_code.len());
                code.extend(expr_code);
            }

            if let Some(var) = potential_var {
                // call a method
                let class_name = match &var.0 {
                    VarKind::Static(t) | VarKind::Field(t) | VarKind::Arg(t) | VarKind::Var(t) => {
                        match t {
                            crate::parser::Type::ClassName(classname) => classname,
                            crate::parser::Type::Int => bail!("`int` type has no method"),
                            crate::parser::Type::Char => bail!("`char` type has no method"),
                            crate::parser::Type::Boolean => bail!("`boolean` type has no method"),
                        }
                    }
                };
                code.reserve_exact(1);
                code.push(format_compact!(
                    "call {}.{} {}",
                    class_name,
                    func_name,
                    1 + expr_list.len()
                ));
            } else {
                // call a function/constructor
                code.reserve_exact(1);
                code.push(format_compact!(
                    "call {}.{} {}",
                    name,
                    func_name,
                    expr_list.len()
                ));
            }

            code
        }
    })
}

fn get_symbol_table<'a>(
    class_table: &'a ClassSymbolTable,
    args: &ParameterLists,
    vars: &Vec<VarDeclaration>,
    this: Option<ClassName>,
    filter_field: bool,
) -> SymbolTable<'a> {
    SymbolTable::new(
        class_table,
        LocalSymbolTable::new(args, vars, this),
        filter_field,
    )
}

fn get_segment_name(kind: &VarKind) -> &'static str {
    match kind {
        VarKind::Static(_) => "static",
        VarKind::Field(_) => "this",
        VarKind::Arg(_) => "argument",
        VarKind::Var(_) => "local",
    }
}

fn get_binary_op_ins(binary_op: &BinaryOp) -> &'static str {
    match binary_op {
        BinaryOp::Add => "add",
        BinaryOp::Minus => "sub",
        BinaryOp::Mult => "call Math.multiply 2",
        BinaryOp::Div => "call Math.divide 2",
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
        BinaryOp::Lt => "lt",
        BinaryOp::Gt => "gt",
        BinaryOp::Eq => "eq",
    }
}
