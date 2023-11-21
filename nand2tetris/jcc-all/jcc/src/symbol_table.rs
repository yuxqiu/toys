use std::collections::HashMap;

use compact_str::CompactString;
use delegate::delegate;
use getset::Getters;

use crate::parser::{ClassName, ClassVarDeclaration, ParameterLists, VarDeclaration};

pub type Type = crate::parser::Type;

pub enum VarKind {
    Static(Type),
    Field(Type),
    Arg(Type),
    Var(Type),
}
pub struct ClassSymbolTable(HashMap<CompactString, (VarKind, u16)>);
pub struct LocalSymbolTable(HashMap<CompactString, (VarKind, u16)>);

impl ClassSymbolTable {
    pub fn new(vars: &Vec<ClassVarDeclaration>) -> ClassSymbolTable {
        let mut symbol_table = HashMap::new();
        symbol_table.reserve(vars.len());

        let mut static_count = 0;
        let mut field_count = 0;

        vars.iter().for_each(|var| match var {
            ClassVarDeclaration::Static(decl) => {
                symbol_table.insert(
                    decl.var_name().clone(),
                    (VarKind::Static(decl.var_type().clone()), static_count),
                );
                static_count += 1;
            }
            ClassVarDeclaration::Field(decl) => {
                symbol_table.insert(
                    decl.var_name().clone(),
                    (VarKind::Field(decl.var_type().clone()), field_count),
                );
                field_count += 1;
            }
        });

        ClassSymbolTable(symbol_table)
    }

    delegate! {
        to self.0 {
            #[call(len)]
            pub fn len(&self) -> usize;
        }
    }
}

impl LocalSymbolTable {
    pub fn new(
        args: &ParameterLists,
        vars: &Vec<VarDeclaration>,
        this: Option<ClassName>,
    ) -> LocalSymbolTable {
        let mut symbol_table = HashMap::new();
        symbol_table.reserve(args.len() + vars.len() + this.is_some() as usize);

        let mut args_count = 0;
        let mut vars_count = 0;

        if let Some(classname) = this {
            symbol_table.insert("this".into(), (VarKind::Arg(Type::ClassName(classname)), 0));
            args_count = 1;
        }

        args.iter().for_each(|decl| {
            symbol_table.insert(
                decl.var_name().clone(),
                (VarKind::Arg(decl.var_type().clone()), args_count),
            );
            args_count += 1;
        });

        vars.iter().for_each(|decl| {
            symbol_table.insert(
                decl.var_name().clone(),
                (VarKind::Var(decl.var_type().clone()), vars_count),
            );
            vars_count += 1;
        });

        LocalSymbolTable(symbol_table)
    }
}

#[derive(Getters)]
#[get = "pub"]
pub struct SymbolTable<'a> {
    class_table: &'a ClassSymbolTable,
    local_table: LocalSymbolTable,
    filter_field: bool,
}

impl<'a> SymbolTable<'a> {
    pub fn new(
        class_table: &ClassSymbolTable,
        local_table: LocalSymbolTable,
        filter_field: bool,
    ) -> SymbolTable {
        SymbolTable {
            class_table,
            local_table,
            filter_field,
        }
    }

    pub fn get(&self, name: &str) -> Option<&(VarKind, u16)> {
        let local = self.local_table.0.get(name);

        if local.is_some() {
            return local;
        }

        let class = self.class_table.0.get(name);

        if class.is_some_and(|var| self.filter_field && matches!(var.0, VarKind::Field(_))) {
            return None;
        }

        class
    }
}
