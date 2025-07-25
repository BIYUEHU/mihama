use crate::{
    checker::{
        ast::{
            CheckedCase, CheckedExpr, CheckedStmt, CheckedTypeVariant, CheckedTypeVariantFields,
        },
        object::TypeObject,
    },
    parser::ast::{Expr, Kind, Stmt, TypeExpr, TypeVariantFields},
};

pub fn is_uppercase_first_letter(str: &str) -> bool {
    str.chars().next().map_or(false, |c| c.is_uppercase())
}

pub fn get_arrow_type(params: Vec<TypeExpr>, return_type: TypeExpr) -> TypeExpr {
    match params.len() {
        0 => TypeExpr::Arrow(
            Box::new(TypeExpr::Con("Unit".to_string())),
            Box::new(return_type),
        ),
        1 => TypeExpr::Arrow(Box::new(params[0].clone()), Box::new(return_type)),
        _ => TypeExpr::Arrow(
            Box::new(params[0].clone()),
            Box::new(get_arrow_type(params[1..].to_vec(), return_type)),
        ),
    }
}

pub fn type_to_vec(func_type: &TypeObject) -> Vec<TypeObject> {
    match func_type {
        TypeObject::Function(param_type, return_type) => {
            [type_to_vec(param_type), type_to_vec(return_type)].concat()
        }
        _ => vec![func_type.clone()],
    }
}

pub fn vec_to_type(vec_type: Vec<TypeObject>) -> TypeObject {
    if vec_type.len() == 1 {
        vec_type[0].clone()
    } else {
        TypeObject::Function(
            Box::new(vec_to_type(vec_type[0..vec_type.len() - 1].to_vec())),
            Box::new(vec_to_type(vec_type[vec_type.len() - 1..].to_vec())),
        )
    }
}

pub fn vec_to_kind(vec_kind: Vec<Kind>) -> Kind {
    if vec_kind.len() == 1 {
        vec_kind[0].clone()
    } else {
        Kind::Arrow(
            Box::new(vec_to_kind(vec_kind[0..vec_kind.len() - 1].to_vec())),
            Box::new(vec_to_kind(vec_kind[vec_kind.len() - 1..].to_vec())),
        )
    }
}

pub fn num_to_kind(num: usize) -> Kind {
    match num {
        0 => Kind::Star,
        _ => Kind::Arrow(Box::new(Kind::Star), Box::new(num_to_kind(num - 1))),
    }
}

pub fn is_op_char(ch: char) -> bool {
    matches!(
        ch,
        '+' | '-'
            | '*'
            | '/'
            | '%'
            | '='
            | '<'
            | '>'
            | '!'
            | '&'
            | '|'
            | '^'
            | ':'
            | '$'
            | '?'
            | '.'
    )
}

pub fn to_checked_expr(expr: Expr) -> CheckedExpr {
    match expr {
        Expr::Ident(name) => CheckedExpr::Ident {
            value: name,
            type_annotation: TypeObject::Any,
        },
        Expr::Internal(name) => CheckedExpr::Internal {
            value: name,
            type_annotation: TypeObject::Any,
        },
        Expr::Literal(lit) => CheckedExpr::Literal {
            value: lit,
            type_annotation: TypeObject::Any,
        },
        Expr::Prefix(op, sub) => CheckedExpr::Prefix {
            op,
            expr: Box::new(to_checked_expr(*sub)),
            type_annotation: TypeObject::Any,
        },
        Expr::Infix(op, left, right) => CheckedExpr::Infix {
            op,
            left: Box::new(to_checked_expr(*left)),
            right: Box::new(to_checked_expr(*right)),
            type_annotation: TypeObject::Any,
        },
        Expr::Call { callee, params } => CheckedExpr::Call {
            callee: Box::new(to_checked_expr(*callee)),
            params: params
                .into_iter()
                .map(|param| to_checked_expr(param))
                .collect::<Vec<_>>(),
            type_annotation: TypeObject::Any,
        },
        Expr::Function { params, body, .. } => CheckedExpr::Function {
            params: params.into_iter().map(|(name, _)| name).collect(),
            body: Box::new(to_checked_expr(*body)),
            type_annotation: TypeObject::Any,
        },
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => CheckedExpr::If {
            condition: Box::new(to_checked_expr(*condition)),
            then_branch: Box::new(to_checked_expr(*then_branch)),
            else_branch: Box::new(to_checked_expr(*else_branch)),
            type_annotation: TypeObject::Any,
        },
        Expr::Match { expr, cases } => CheckedExpr::Match {
            expr: Box::new(to_checked_expr(*expr)),
            cases: {
                cases
                    .into_iter()
                    .map(|case| CheckedCase {
                        pattern: case.pattern.clone(),
                        guard: to_checked_expr(case.guard.clone()),
                        body: to_checked_expr(case.body),
                    })
                    .collect::<Vec<_>>()
            },
            type_annotation: TypeObject::Any,
        },
        Expr::LetIn {
            name, value, body, ..
        } => CheckedExpr::LetIn {
            name,
            value: Box::new(to_checked_expr(*value)),
            body: Box::new(to_checked_expr(*body)),
            type_annotation: TypeObject::Any,
        },
        Expr::Block(stmts) => CheckedExpr::Block {
            stmts: stmts
                .into_iter()
                .map(|stmt| to_checked_stmt(stmt))
                .collect::<Vec<_>>(),
            type_annotation: TypeObject::Any,
        },
    }
}

pub fn to_checked_stmt(stmt: Stmt) -> CheckedStmt {
    match stmt.clone() {
        Stmt::Let { name, value, .. } => CheckedStmt::Let {
            name: name.clone(),
            type_annotation: TypeObject::Any,
            value: to_checked_expr(*value),
        },
        Stmt::Type {
            name,
            params,
            variants,
            ..
        } => CheckedStmt::Type {
            name: name.clone(),
            params: params.clone(),
            kind_annotation: Kind::Star,
            variants: {
                variants
                    .iter()
                    .map(|variant| match variant.fields.clone() {
                        TypeVariantFields::Unit => CheckedTypeVariant {
                            name: variant.name.clone(),
                            fields: CheckedTypeVariantFields::Unit,
                        },
                        TypeVariantFields::Tuple(fields) => CheckedTypeVariant {
                            name: variant.name.clone(),
                            fields: CheckedTypeVariantFields::Tuple(
                                fields.iter().map(|_| TypeObject::Any).collect::<Vec<_>>(),
                            ),
                        },
                        _ => unimplemented!("record type variant"),
                    })
                    .collect::<Vec<_>>()
            },
        },
        Stmt::ImportAll { source, alias } => CheckedStmt::ImportAll { source, alias },
        Stmt::ImportSome { source, items } => CheckedStmt::ImportSome { source, items },
        Stmt::Export {
            body,
            only_abstract,
        } => CheckedStmt::Export {
            body: Box::new(to_checked_stmt(*body)),
            only_abstract: only_abstract,
        },
        Stmt::Expr(expr) => CheckedStmt::Expr(to_checked_expr(expr)),
    }
}
