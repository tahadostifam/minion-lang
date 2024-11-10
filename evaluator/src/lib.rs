use ast::{
    expression::{
        Boolean, Expression, FunctionCall, Identifier, Integer, Literal, StringType,
        UnaryExpression,
    },
    statement::{BlockStatement, Function, If, Return, Statement},
    Node,
};
use object::{
    builtins::BuiltIns,
    env::{Env, Environment},
    object::{EvalError, Object},
};
use std::{cell::RefCell, rc::Rc};
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
        result = eval_statement(&statement, env)?;
    }

    Ok(result)
}

fn eval_statement(statement: &Statement, env: &Env) -> Result<Rc<Object>, EvalError> {
    match statement {
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
        Statement::Function(Function { params, body, .. }) => {
            eval_function_statement(params.clone(), *body.clone(), &env.clone())
        }
    }
}

fn eval_function_statement(
    params: Vec<Identifier>,
    body: BlockStatement,
    env: &Env,
) -> Result<Rc<Object>, EvalError> {
    Ok(Rc::new(Object::Function(params, body, env.clone())))
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

fn eval_expression(expr: Expression, env: &Env) -> Result<Rc<Object>, EvalError> {
    match expr {
        Expression::Literal(literal) => eval_literal(&literal),
        Expression::Identifier(identifier) => eval_identifier(identifier.name.as_str(), env),
        Expression::Prefix(UnaryExpression {
            operator, operand, ..
        }) => {
            let val = eval_expression(*operand, &Rc::clone(env))?;
            return eval_prefix(operator.kind, &val);
        }
        Expression::Infix(binary_expression) => {
            let left = eval_expression(*binary_expression.left, &Rc::from(env.clone()))?;
            let right = eval_expression(*binary_expression.right, &Rc::from(env.clone()))?;
            return eval_infix(binary_expression.operator, &left, &right);
        }
        Expression::FunctionCall(FunctionCall {
            call, arguments, ..
        }) => {
            let func = eval_expression(*call, &Rc::clone(env))?;
            let args = eval_expressions(&arguments, env)?;
            dbg!(func.clone());
            dbg!(args.clone());
            apply_function(&func, &args)
        }
    }
}

fn apply_function(function: &Rc<Object>, args: &Vec<Rc<Object>>) -> Result<Rc<Object>, EvalError> {
    match &**function {
        Object::Function(params, body, env) => {
            let mut env = Environment::new_enclosed_environment(&env);

            params.iter().enumerate().for_each(|(i, param)| {
                env.set(param.name.clone(), args[i].clone());
            });

            let evaluated = eval_block_statements(&body.body, &Rc::new(RefCell::new(env)))?;
            return unwrap_return(evaluated);
        }
        Object::Builtin(b) => Ok(b(args.to_vec())),
        f => Err(format!("expected {} to be a function", f)),
    }
}

fn eval_identifier(identifier: &str, env: &Env) -> Result<Rc<Object>, EvalError> {
    match env.borrow().get(identifier) {
        Some(obj) => Ok(obj),
        None => match BuiltIns.iter().find(|&&b| b.0 == identifier) {
            Some(obj) => Ok(Rc::new(Object::Builtin(obj.1))),
            None => Err(format!("unknown identifier {}", identifier)),
        },
    }
}

fn unwrap_return(obj: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    if let Object::ReturnValue(val) = &*obj {
        Ok(Rc::clone(&val))
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
            return eval_integer_infix(operator.kind, *left, *right);
        }
        (Object::Boolean(left), Object::Boolean(right)) => {
            return eval_boolean_infix(operator.kind, *left, *right);
        }
        (Object::String(left), Object::String(right)) => {
            return eval_string_infix(operator.kind, left, right);
        }
        _ => {
            return Err(format!(
                "eval infix not available for operator: {}",
                operator.kind
            ))
        }
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
        Object::Null => return false,
        Object::Boolean(false) => return false,
        _ => true,
    }
}
