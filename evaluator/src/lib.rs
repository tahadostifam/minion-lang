use ast::{
    expression::{
        Boolean, Expression, FunctionCall, Identifier, Integer, Literal, StringType,
        UnaryExpression, UnaryOperator,
    },
    statement::{BlockStatement, Function, If, Return, Statement, Variable},
    Node,
};
use builtins::BUILT_INS;
use object::{
    env::{Env, Environment},
    object::{EvalError, Object},
};
use std::{alloc::GlobalAlloc, borrow::Borrow, cell::RefCell, rc::Rc};
use token::{Token, TokenKind};

mod evaluator_test;

pub fn eval(node: Node, env: &Env) -> Result<Rc<Object>, EvalError> {
    match node {
        Node::Program(program) => eval_block_statements(&program.body, env),
        Node::Statement(statement) => eval_statement(&statement, env),
        Node::Expression(expression) => eval_expression(expression, env),
    }
}

fn eval_block_statements(statements: &Vec<Statement>, env: &Env) -> Result<Rc<Object>, EvalError> {
    let mut result = Rc::new(Object::Null);

    for statement in statements {
        result = eval_statement(statement, env)?;
    }

    Ok(result)
}

fn eval_statement(statement: &Statement, env: &Env) -> Result<Rc<Object>, EvalError> {
    match statement {
        Statement::For(for_stmt) => eval_for_statement(
            for_stmt.initializer.clone(),
            for_stmt.condition.clone(),
            for_stmt.increment.clone(),
            for_stmt.body.clone(),
            &env.clone(),
        ),
        Statement::VariableDeclaration(variable) => {
            eval_variable_declaration(&variable.identifier, variable.expr.clone(), env)
        }
        Statement::Expression(expression) => eval_expression(expression.clone(), env),
        Statement::If(If {
            condition,
            consequent,
            alternate,
            branches,
            ..
        }) => eval_if_statement(condition, consequent, alternate, branches, env),
        Statement::Return(Return { argument, .. }) => eval_return_statement(argument, env),
        Statement::Function(Function {
            name, params, body, ..
        }) => eval_function_statement(name.clone(), params.clone(), *body.clone(), &env.clone()),
    }
}

fn eval_for_statement(
    initializer: Option<Variable>,
    condition: Option<Expression>,
    increment: Option<Expression>,
    body: Box<BlockStatement>,
    env: &Env,
) -> Result<Rc<Object>, EvalError> {
    todo!();
    // if let Some(var) = initializer {
    //     eval_variable_declaration(&var.identifier.clone(), var.expr.clone(), env)?;
    // }

    // loop {
    //     eval_block_statements(&body.body, env);

    //     if let Some(expr) = condition {
    //         eval_expression(condition, env)
    //     }
    //     break;
    // }

    // Ok(Rc::new(Object::Null))
}

fn eval_function_statement(
    name: String,
    params: Vec<Identifier>,
    body: BlockStatement,
    env: &Env,
) -> Result<Rc<Object>, EvalError> {
    // we prevent overwriting built-in functions!
    match BUILT_INS.borrow().get(name.as_str()) {
        Some(_) => Err(format!(
            "redeclaring built-in function {} is not allowed",
            name
        )),
        None => {
            let declare_fn = Rc::new(Object::Function(params, body, env.clone()));
            env.borrow_mut().set(name, declare_fn.clone());
            Ok(declare_fn)
        }
    }
}

fn eval_return_statement(argument: &Expression, env: &Env) -> Result<Rc<Object>, EvalError> {
    Ok(Rc::new(Object::ReturnValue(eval_expression(
        argument.clone(),
        env,
    )?)))
}

fn eval_if_statement(
    condition: &Expression,
    consequent: &BlockStatement,
    alternate: &Option<Box<BlockStatement>>,
    branches: &Vec<If>,
    env: &Env,
) -> Result<Rc<Object>, EvalError> {
    let condition = eval_expression(condition.clone(), &Rc::clone(env))?;

    if is_truthy(&condition) {
        eval_block_statements(&(consequent.body), env)
    } else {
        for stmt in branches {
            let condition = eval_expression(stmt.condition.clone(), env)?;

            if is_truthy(&condition) {
                return eval_block_statements(&stmt.consequent.body, env);
            } else {
                continue;
            }
        }

        match alternate {
            Some(alt) => eval_block_statements(&(alt.body), env),
            None => Ok(Rc::new(Object::Null)),
        }
    }
}

fn eval_expressions(exprs: &Vec<Expression>, env: &Env) -> Result<Vec<Rc<Object>>, EvalError> {
    let mut list = Vec::new();
    for expr in exprs {
        let val = eval_expression(expr.clone(), &Rc::clone(env))?;
        list.push(val);
    }

    Ok(list)
}

fn validate_func_args_len(params_len: usize, args_len: usize) -> Result<(), EvalError> {
    if params_len != args_len {
        Err(format!(
            "wrong number of arguments! wanted {} got {}",
            params_len, args_len
        ))
    } else {
        Ok(())
    }
}

fn eval_expression(expr: Expression, env: &Env) -> Result<Rc<Object>, EvalError> {
    match expr {
        Expression::UnaryOperator(unop) => eval_unary_operator(unop, env),
        Expression::FunctionCall(FunctionCall {
            call, arguments, ..
        }) => match *call {
            Expression::Identifier(Identifier { name, .. }) => {
                // Let's distinguish the built-in funcs and declared ones

                let args = eval_expressions(&arguments, env)?;

                match BUILT_INS.borrow().get(name.as_str()) {
                    Some(bfn) => Ok(bfn(args)),
                    None => {
                        let func = env
                            .borrow_mut()
                            .get(&name)
                            .unwrap_or_else(|| panic!("{} not declared", name));

                        let result = match &*func {
                            Object::Function(params, body, env) => {
                                let mut env = Environment::new_enclosed_environment(env);

                                if let Err(e) = validate_func_args_len(params.len(), args.len()) {
                                    return Err(e);
                                }

                                params.iter().enumerate().for_each(|(i, param)| {
                                    env.set(param.name.clone(), args[i].clone());
                                });

                                let evaluated =
                                    eval_block_statements(&body.body, &Rc::new(RefCell::new(env)))?;

                                return unwrap_return(evaluated);
                            }
                            f => Err(format!("expected {} to be a function", f)),
                        };

                        result
                    }
                }
            }
            _ => Err(format!(
                "expected to get function declaratin from the object store but got {:?}",
                call
            )),
        },
        Expression::Literal(literal) => eval_literal(&literal),
        Expression::Identifier(identifier) => eval_identifier(identifier.name.as_str(), env),
        Expression::Prefix(UnaryExpression {
            operator, operand, ..
        }) => {
            let val = eval_expression(*operand, &Rc::clone(env))?;
            eval_prefix(operator.kind, &val)
        }
        Expression::Infix(binary_expression) => {
            let left = eval_expression(*binary_expression.left, &Rc::clone(env))?;
            let right = eval_expression(*binary_expression.right, &Rc::clone(env))?;
            eval_infix(binary_expression.operator, &left, &right)
        }
    }
}

fn eval_unary_operator(unop: UnaryOperator, env: &Env) -> Result<Rc<Object>, EvalError> {
    let mut scope = env.borrow_mut();
    let object = scope.get(&unop.identifer.name);

    if let Some(var) = object {
        match *var {
            Object::Integer(value) => {
                let new_value = Rc::new(Object::Integer(match unop.ty {
                    ast::expression::UnaryOperatorType::PreIncrement => value + 1,
                    ast::expression::UnaryOperatorType::PostIncrement => value + 1,
                    ast::expression::UnaryOperatorType::PreDecrement => value - 1,
                    ast::expression::UnaryOperatorType::PostDecrement => value - 1,
                }));

                match unop.ty {
                    ast::expression::UnaryOperatorType::PreIncrement
                    | ast::expression::UnaryOperatorType::PreDecrement => {
                        scope.set(unop.identifer.name, new_value.clone());
                        return Ok(new_value);
                    }
                    ast::expression::UnaryOperatorType::PostIncrement
                    | ast::expression::UnaryOperatorType::PostDecrement => {
                        let temp = var;
                        scope.set(unop.identifer.name, new_value.clone());
                        return Ok(temp);
                    }
                };
            }
            _ => {
                return Err(format!(
                    "unary operation can only performed for number objects but got {}",
                    var
                ))
            }
        }
    } else {
        return Err(format!(
            "variable {} is not initialized and can not be operated",
            unop.identifer.name
        ));
    }
}

fn eval_identifier(identifier: &str, env: &Env) -> Result<Rc<Object>, EvalError> {
    match env.borrow_mut().get(identifier) {
        Some(obj) => Ok(obj),
        None => match BUILT_INS.borrow().get(identifier) {
            Some(obj) => Ok(Rc::new(Object::Builtin(*obj))),
            None => Err(format!("unknown identifier {}", identifier)),
        },
    }
}

fn unwrap_return(obj: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    if let Object::ReturnValue(val) = &*obj {
        Ok(Rc::clone(val))
    } else {
        Ok(obj)
    }
}

fn eval_variable_declaration(
    identifier: &Token,
    expr: Expression,
    env: &Env,
) -> Result<Rc<Object>, EvalError> {
    let val = eval_expression(expr, env)?;
    let obj: Rc<Object> = Rc::clone(&val);

    if let TokenKind::Identifier { name } = &identifier.kind {
        env.borrow_mut().set(name.clone(), obj);
    }

    Ok(Rc::new(Object::Null))
}

fn eval_infix(operator: Token, left: &Object, right: &Object) -> Result<Rc<Object>, EvalError> {
    match (left, right) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_integer_infix(operator.kind, *left, *right)
        }
        (Object::Boolean(left), Object::Boolean(right)) => {
            eval_boolean_infix(operator.kind, *left, *right)
        }
        (Object::String(left), Object::String(right)) => {
            eval_string_infix(operator.kind, left, right)
        }
        (Object::String(left), Object::Integer(right)) => {
            eval_string_infix(operator.kind, left, &right.to_string())
        }
        (Object::Integer(left), Object::String(right)) => {
            eval_string_infix(operator.kind, &left.to_string(), right)
        }
        (Object::Boolean(left), Object::String(right)) => {
            eval_string_infix(operator.kind, &left.to_string(), right)
        }
        (Object::String(left), Object::Boolean(right)) => {
            eval_string_infix(operator.kind, left, &right.to_string())
        }
        _ => Err(format!(
            "eval infix not available for operator: {}",
            operator.kind
        )),
    }
}

fn eval_boolean_infix(
    operator: TokenKind,
    left: bool,
    right: bool,
) -> Result<Rc<Object>, EvalError> {
    let result = match operator {
        TokenKind::Equal => Object::Boolean(left == right),
        TokenKind::NotEqual => Object::Boolean(left != right),
        op => return Err(format!("invalid infix operator for boolean: {}", op)),
    };

    Ok(Rc::from(result))
}

fn eval_string_infix(
    operator: TokenKind,
    left: &String,
    right: &String,
) -> Result<Rc<Object>, EvalError> {
    let result = match operator {
        TokenKind::Else => Object::Boolean(left == right),
        TokenKind::NotEqual => Object::Boolean(left != right),
        TokenKind::Plus => Object::String(format!("{}{}", left, right)),
        op => return Err(format!("invalid infix {} operator for string", op)),
    };

    Ok(Rc::from(result))
}

fn eval_integer_infix(operator: TokenKind, left: i64, right: i64) -> Result<Rc<Object>, EvalError> {
    let result = match operator {
        TokenKind::Plus => Object::Integer(left + right),
        TokenKind::Minus => Object::Integer(left - right),
        TokenKind::Asterisk => Object::Integer(left * right),
        TokenKind::Slash => Object::Integer(left / right),
        TokenKind::LessThan => Object::Boolean(left < right),
        TokenKind::GreaterThan => Object::Boolean(left > right),
        TokenKind::LessEqual => Object::Boolean(left <= right),
        TokenKind::GreaterEqual => Object::Boolean(left >= right),
        TokenKind::Equal => Object::Boolean(left == right),
        TokenKind::NotEqual => Object::Boolean(left != right),
        op => return Err(format!("invalid infix operator {} for integer", op)),
    };

    Ok(Rc::from(result))
}

fn eval_literal(literal: &Literal) -> Result<Rc<Object>, EvalError> {
    let result = match literal {
        Literal::Integer(Integer { raw: i, .. }) => Rc::from(Object::Integer(*i)),
        Literal::Boolean(Boolean { raw: b, .. }) => Rc::from(Object::Boolean(*b)),
        Literal::String(StringType { raw: s, .. }) => Rc::from(Object::String(s.clone())),
    };

    Ok(result)
}

fn eval_prefix(operator: TokenKind, right: &Object) -> Result<Rc<Object>, EvalError> {
    match operator {
        TokenKind::Bang => eval_prefix_bang(right),
        TokenKind::Minus => eval_prefix_minus(right),
        _ => Err(format!("unknown prefix operator: {}", operator)),
    }
}

fn eval_prefix_bang(expr: &Object) -> Result<Rc<Object>, EvalError> {
    match *expr {
        Object::Null => Ok(Rc::new(Object::Boolean(true))),
        Object::Boolean(b) => Ok(Rc::new(Object::Boolean(!b))),
        _ => Ok(Rc::new(Object::Boolean(false))),
    }
}

fn eval_prefix_minus(expr: &Object) -> Result<Rc<Object>, EvalError> {
    match *expr {
        Object::Integer(i) => Ok(Rc::from(Object::Integer(-i))),
        _ => Err(format!("can't apply prefix minus operator: {}", expr)),
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(false) => false,
        _ => true,
    }
}
