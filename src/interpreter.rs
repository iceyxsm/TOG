use crate::ast::*;
use crate::error::TogError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum ControlFlow {
    Normal,
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Struct {
        name: String,
        fields: HashMap<String, Value>,
    },
    Function {
        name: String,
        params: Vec<Param>,
        body: Expr,
        bound_self: Option<Box<Value>>,
    },
    None,
}

pub struct Interpreter {
    environment: HashMap<String, Value>,
    struct_defs: HashMap<String, (Vec<(String, Option<Type>)>, Vec<MethodDecl>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
            struct_defs: HashMap::new(),
        }
    }
    
    pub fn interpret(program: Program) -> Result<(), TogError> {
        let mut interpreter = Self::new();
        
        // First pass: define all functions
        for stmt in program.statements {
            let (_, _) = interpreter.execute_stmt(&stmt)?;
        }
        
        // Second pass: automatically call main() if it exists
        if let Some(Value::Function { .. }) = interpreter.environment.get("main") {
            let main_func = interpreter.environment.get("main").unwrap().clone();
            if let Value::Function { body, .. } = main_func {
                interpreter.evaluate(&body)?;
            }
        }
        
        Ok(())
    }
    
    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<(Value, ControlFlow), TogError> {
        match stmt {
            Stmt::Expr(expr) => {
                let val = self.evaluate(expr)?;
                Ok((val, ControlFlow::Normal))
            }
            Stmt::Let { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.environment.insert(name.clone(), val.clone());
                Ok((val, ControlFlow::Normal))
            }
            Stmt::Assign { name, value } => {
                // Check if variable exists
                if !self.environment.contains_key(name) {
                    return Err(TogError::RuntimeError(
                        format!("Cannot assign to undefined variable: {}", name),
                        None
                    ));
                }
                let val = self.evaluate(value)?;
                self.environment.insert(name.clone(), val.clone());
                Ok((val, ControlFlow::Normal))
            }
            Stmt::AssignField { object, field, value } => {
                // Support nested field assignment: obj.field = value where obj can be nested access
                let new_val = self.evaluate(value)?;
                self.assign_field_chain(object, field, new_val)?;
                Ok((Value::None, ControlFlow::Normal))
            }
            Stmt::StructDef { name, fields, methods } => {
                self.struct_defs.insert(name.clone(), (fields.clone(), methods.clone()));
                Ok((Value::None, ControlFlow::Normal))
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    let val = self.evaluate(expr)?;
                    Ok((val, ControlFlow::Normal))
                } else {
                    Ok((Value::None, ControlFlow::Normal))
                }
            }
            Stmt::Break => {
                Ok((Value::None, ControlFlow::Break))
            }
            Stmt::Continue => {
                Ok((Value::None, ControlFlow::Continue))
            }
        }
    }
    
    fn evaluate_block_with_control_flow(&mut self, body: &Expr) -> Result<ControlFlow, TogError> {
        match body {
            Expr::Block(statements) => {
                for stmt in statements {
                    // Check if this statement is an expression that might contain control flow
                    match stmt {
                        Stmt::Expr(expr) => {
                            // Recursively evaluate with control flow awareness
                            match self.evaluate_block_with_control_flow(expr) {
                                Ok(ControlFlow::Break) | Ok(ControlFlow::Continue) => {
                                    return Ok(self.evaluate_block_with_control_flow(expr)?);
                                }
                                Ok(ControlFlow::Normal) => {}
                                Err(e) => return Err(e),
                            }
                        }
                        _ => {
                            // Regular statement - execute normally
                            let (_, flow) = self.execute_stmt(stmt)?;
                            match flow {
                                ControlFlow::Break | ControlFlow::Continue => {
                                    return Ok(flow);
                                }
                                ControlFlow::Normal => {}
                            }
                        }
                    }
                }
                Ok(ControlFlow::Normal)
            }
            Expr::If { condition, then_branch, else_branch } => {
                // Handle if statements with control flow
                let cond_val = self.evaluate(condition)?;
                if is_truthy(&cond_val) {
                    self.evaluate_block_with_control_flow(then_branch)
                } else if let Some(else_expr) = else_branch {
                    self.evaluate_block_with_control_flow(else_expr)
                } else {
                    Ok(ControlFlow::Normal)
                }
            }
            _ => {
                // Single expression body - evaluate it (can't have break/continue)
                self.evaluate(body)?;
                Ok(ControlFlow::Normal)
            }
        }
    }
    
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, TogError> {
        match expr {
            Expr::Literal(lit) => {
                match lit {
                    Literal::Array(elems) => {
                        let mut values = Vec::new();
                        for elem in elems {
                            values.push(self.evaluate(elem)?);
                        }
                        Ok(Value::Array(values))
                    }
                    _ => Ok(literal_to_value(lit)),
                }
            }
            Expr::StructLiteral { name, fields } => {
                let def = self.struct_defs.get(name).cloned()
                    .ok_or_else(|| TogError::RuntimeError(
                        format!("Unknown struct: {}", name),
                        None
                    ))?;
                let (field_defs, _) = def;
                // Build field map
                let mut map = HashMap::new();
                for (field_name, expr) in fields {
                    let val = self.evaluate(expr)?;
                    map.insert(field_name.clone(), val);
                }
                // Basic validation: ensure required fields present
                for (fname, _) in &field_defs {
                    if !map.contains_key(fname) {
                        return Err(TogError::RuntimeError(
                            format!("Missing field '{}' in struct literal {}", fname, name),
                            None
                        ));
                    }
                }
                Ok(Value::Struct {
                    name: name.clone(),
                    fields: map,
                })
            }
            Expr::Variable(name) => {
                // Builtin functions are handled in call expressions
                self.environment.get(name)
                    .cloned()
                    .ok_or_else(|| TogError::RuntimeError(
                        format!("Undefined variable: {}", name),
                        None
                    ))
            }
            Expr::FieldAccess { object, field } => {
                let obj_val = self.evaluate(object)?;
                match obj_val {
                    Value::Struct { fields, .. } => {
                        fields.get(field)
                            .cloned()
                            .ok_or_else(|| TogError::RuntimeError(
                                format!("Field '{}' not found", field),
                                None
                            ))
                    }
                    _ => Err(TogError::RuntimeError(
                        "Field access on non-struct value".to_string(),
                        None
                    ))
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.evaluate_binary_op(&left_val, *op, &right_val)
            }
            Expr::UnaryOp { op, expr } => {
                let val = self.evaluate(expr)?;
                self.evaluate_unary_op(*op, &val)
            }
            Expr::Call { callee, args } => {
                let arg_values: Result<Vec<Value>, TogError> = 
                    args.iter().map(|arg| self.evaluate(arg)).collect();
                let arg_values = arg_values?;
                
                // Method call: obj.method(...)
                if let Expr::FieldAccess { object, field: method_name } = callee.as_ref() {
                    let obj_val = self.evaluate(object)?;
                    if let Value::Struct { name: struct_name, .. } = obj_val.clone() {
                        if let Some((_, methods)) = self.struct_defs.get(&struct_name) {
                            if let Some(method) = methods.iter().find(|m| m.name == *method_name).cloned() {
                                if arg_values.len() != method.params.len() {
                                    return Err(TogError::RuntimeError(
                                        format!("Method {} expects {} arguments, got {}", method_name, method.params.len(), arg_values.len()),
                                        None
                                    ));
                                }
                                
                                let old_env = self.environment.clone();
                                // New scope
                                self.environment = HashMap::new();
                                self.environment.insert("self".to_string(), obj_val.clone());
                                for (param, arg_value) in method.params.iter().zip(arg_values.iter()) {
                                    self.environment.insert(param.name.clone(), arg_value.clone());
                                }
                                let result = self.evaluate(&method.body);
                                self.environment = old_env;
                                return result;
                            }
                        }
                        return Err(TogError::RuntimeError(
                            format!("Unknown method '{}' on struct {}", method_name, struct_name),
                            None
                        ));
                    }
                }
                
                // Check for builtin functions first
                if let Expr::Variable(name) = callee.as_ref() {
                    match name.as_str() {
                        "print" => {
                            // print is now a builtin function
                            for arg in &arg_values {
                                print!("{}", value_to_string(arg));
                            }
                            println!(); // Newline after print
                            return Ok(Value::None);
                        }
                        _ => {
                match crate::stdlib::call_builtin(name, &arg_values) {
                                Ok(result) => return Ok(result),
                                Err(TogError::RuntimeError(ref msg, _)) if msg.contains("Unknown builtin") => {
                                    // Not a builtin, continue to normal evaluation
                                }
                                Err(e) => return Err(e), // Other error (wrong args, etc.)
                            }
                        }
                    }
                }
                
                let callee_val = self.evaluate(callee)?;
                match callee_val {
                    Value::Function { params, body, bound_self, .. } => {
                        // Check argument count
                        if arg_values.len() != params.len() {
                            return Err(TogError::RuntimeError(
                                format!(
                                    "Function expects {} arguments, got {}",
                                    params.len(),
                                    arg_values.len()
                                ),
                                None
                            ));
                        }
                        
                        // Save current environment
                        let old_env = self.environment.clone();
                        
                        // Create new scope and bind parameters
                        if let Some(self_val) = bound_self {
                            self.environment.insert("self".to_string(), (*self_val).clone());
                        }
                        for (param, arg_value) in params.iter().zip(arg_values.iter()) {
                            self.environment.insert(param.name.clone(), arg_value.clone());
                        }
                        
                        // Execute function body
                        let result = self.evaluate(&body);
                        
                        // Restore environment (pop scope)
                        self.environment = old_env;
                        
                        result
                    }
                    _ => Err(TogError::RuntimeError(
                        "Can only call functions".to_string(),
                        None
                    ))
                }
            }
            Expr::Block(statements) => {
                // Blocks can contain break/continue, but we can't handle them here
                // They need to be handled by the loop. For now, we'll evaluate normally
                // and let the loop's evaluate_block_with_control_flow handle it
                let mut last_value = Value::None;
                for stmt in statements {
                    let (val, flow) = self.execute_stmt(stmt)?;
                    last_value = val;
                    // If we get break/continue here, it means we're not in a loop context
                    // This will be caught by the loop's evaluate_block_with_control_flow
                    match flow {
                        ControlFlow::Break | ControlFlow::Continue => {
                            // This is an error - break/continue outside loop
                            return Err(TogError::RuntimeError(
                                format!("{:?} outside of loop", flow),
                                None
                            ));
                        }
                        ControlFlow::Normal => {}
                    }
                }
                Ok(last_value)
            }
            Expr::If { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate(condition)?;
                if is_truthy(&cond_val) {
                    // Check if then_branch can have control flow (Block or nested If)
                    match then_branch.as_ref() {
                        Expr::Block(_) | Expr::If { .. } => {
                            // Use control flow aware evaluation, but we're not in a loop
                            // So we'll just evaluate normally and catch any break/continue as errors
                            match self.evaluate_block_with_control_flow(then_branch) {
                                Ok(ControlFlow::Normal) => Ok(Value::None),
                                Ok(ControlFlow::Break) => Err(TogError::RuntimeError("Break outside of loop".to_string(), None)),
                                Ok(ControlFlow::Continue) => Err(TogError::RuntimeError("Continue outside of loop".to_string(), None)),
                                Err(e) => Err(e),
                            }
                        }
                        _ => self.evaluate(then_branch)
                    }
                } else if let Some(else_expr) = else_branch {
                    match else_expr.as_ref() {
                        Expr::Block(_) | Expr::If { .. } => {
                            match self.evaluate_block_with_control_flow(else_expr) {
                                Ok(ControlFlow::Normal) => Ok(Value::None),
                                Ok(ControlFlow::Break) => Err(TogError::RuntimeError("Break outside of loop".to_string(), None)),
                                Ok(ControlFlow::Continue) => Err(TogError::RuntimeError("Continue outside of loop".to_string(), None)),
                                Err(e) => Err(e),
                            }
                        }
                        _ => self.evaluate(else_expr)
                    }
                } else {
                    Ok(Value::None)
                }
            }
            Expr::While { condition, body } => {
                loop {
                    let cond_val = self.evaluate(condition)?;
                    if !is_truthy(&cond_val) {
                        break;
                    }
                    
                    // Evaluate body - handle break/continue
                    let body_result = self.evaluate_block_with_control_flow(body);
                    match body_result {
                        Ok(ControlFlow::Break) => break,
                        Ok(ControlFlow::Continue) => continue,
                        Ok(ControlFlow::Normal) => {},
                        Err(e) => return Err(e),
                    }
                }
                Ok(Value::None)
            }
            Expr::For { variable, iterable, body } => {
                let iterable_val = self.evaluate(iterable)?;
                
                match iterable_val {
                    Value::Array(arr) => {
                        // Save the loop variable's original value (if it exists)
                        let old_loop_var = self.environment.get(variable).cloned();
                        
                        // Iterate over array
                        for item in arr {
                            // Set loop variable for this iteration
                            self.environment.insert(variable.clone(), item);
                            
                            // Execute loop body - handle break/continue
                            let body_result = self.evaluate_block_with_control_flow(body);
                            match body_result {
                                Ok(ControlFlow::Break) => break,
                                Ok(ControlFlow::Continue) => continue,
                                Ok(ControlFlow::Normal) => {},
                                Err(e) => {
                                    // Restore loop variable before returning error
                                    if let Some(old_val) = old_loop_var {
                                        self.environment.insert(variable.clone(), old_val);
                                    } else {
                                        self.environment.remove(variable);
                                    }
                                    return Err(e);
                                }
                            }
                        }
                        
                        // Restore or remove loop variable
                        if let Some(old_val) = old_loop_var {
                            self.environment.insert(variable.clone(), old_val);
                        } else {
                            self.environment.remove(variable);
                        }
                        Ok(Value::None)
                    }
                    Value::String(s) => {
                        // Save the loop variable's original value (if it exists)
                        let old_loop_var = self.environment.get(variable).cloned();
                        
                        for ch in s.chars() {
                            self.environment.insert(variable.clone(), Value::String(ch.to_string()));
                            
                            // Execute loop body - handle break/continue
                            let body_result = self.evaluate_block_with_control_flow(body);
                            match body_result {
                                Ok(ControlFlow::Break) => break,
                                Ok(ControlFlow::Continue) => continue,
                                Ok(ControlFlow::Normal) => {},
                                Err(e) => {
                                    // Restore loop variable before returning error
                                    if let Some(old_val) = old_loop_var {
                                        self.environment.insert(variable.clone(), old_val);
                                    } else {
                                        self.environment.remove(variable);
                                    }
                                    return Err(e);
                                }
                            }
                        }
                        
                        // Restore or remove loop variable
                        if let Some(old_val) = old_loop_var {
                            self.environment.insert(variable.clone(), old_val);
                        } else {
                            self.environment.remove(variable);
                        }
                        Ok(Value::None)
                    }
                    _ => Err(TogError::RuntimeError(
                        format!("Cannot iterate over {:?}, expected array or string", iterable_val),
                        None
                    ))
                }
            }
            Expr::Match { expr, arms } => {
                let value = self.evaluate(expr)?;
                for arm in arms {
                    if self.match_pattern(&arm.pattern, &value)? {
                        // Bind pattern variables to the matched value
                        if let Pattern::Variable(var_name) = &arm.pattern {
                            let old_val = self.environment.get(var_name).cloned();
                            self.environment.insert(var_name.clone(), value.clone());
                            let result = self.evaluate(&arm.body);
                            // Restore old value if it existed
                            if let Some(old) = old_val {
                                self.environment.insert(var_name.clone(), old);
                            } else {
                                self.environment.remove(var_name);
                            }
                            return result;
                        }
                        return self.evaluate(&arm.body);
                    }
                }
                Err(TogError::RuntimeError(
                    "No matching pattern in match expression".to_string(),
                    None
                ))
            }
            Expr::Function { name, params, body, .. } => {
                let func_value = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: *body.clone(),
                    bound_self: None,
                };
                // Store function in environment
                self.environment.insert(name.clone(), func_value.clone());
                Ok(func_value)
            }
            Expr::Index { array, index } => {
                let array_val = self.evaluate(array)?;
                let index_val = self.evaluate(index)?;
                
                match (array_val, index_val) {
                    (Value::Array(arr), Value::Int(idx)) => {
                        if idx < 0 || idx as usize >= arr.len() {
                            return Err(TogError::RuntimeError(
                                format!("Array index {} out of bounds (length: {})", idx, arr.len()),
                                None
                            ));
                        }
                        Ok(arr[idx as usize].clone())
                    }
                    (Value::String(s), Value::Int(idx)) => {
                        if idx < 0 || idx as usize >= s.len() {
                            return Err(TogError::RuntimeError(
                                format!("String index {} out of bounds (length: {})", idx, s.len()),
                                None
                            ));
                        }
                        Ok(Value::String(s.chars().nth(idx as usize).unwrap().to_string()))
                    }
                    (arr, idx) => Err(TogError::RuntimeError(
                        format!("Cannot index {:?} with {:?}", arr, idx),
                        None
                    ))
                }
            }
        }
    }

    fn set_struct_field(struct_val: Value, field: &str, new_val: Value) -> Result<Value, TogError> {
        if let Value::Struct { name, mut fields } = struct_val {
            fields.insert(field.to_string(), new_val);
            Ok(Value::Struct { name, fields })
        } else {
            Err(TogError::RuntimeError(
                format!("Cannot assign field '{}' to non-struct value", field),
                None,
            ))
        }
    }

    fn assign_value_into(&mut self, target: &Expr, replacement: Value) -> Result<(), TogError> {
        match target {
            Expr::Variable(name) => {
                self.environment.insert(name.clone(), replacement);
                Ok(())
            }
            Expr::FieldAccess { object, field } => {
                let parent_val = self.evaluate(object)?;
                let updated_parent = Self::set_struct_field(parent_val, field, replacement)?;
                self.assign_value_into(object, updated_parent)
            }
            _ => Err(TogError::RuntimeError(
                "Invalid assignment target".to_string(),
                None,
            )),
        }
    }

    fn assign_field_chain(&mut self, target: &Expr, field: &str, new_value: Value) -> Result<(), TogError> {
        let obj_val = self.evaluate(target)?;
        let updated_obj = Self::set_struct_field(obj_val, field, new_value)?;
        self.assign_value_into(target, updated_obj)
    }
    
    fn match_pattern(&self, pattern: &Pattern, value: &Value) -> Result<bool, TogError> {
        match (pattern, value) {
            (Pattern::Wildcard, _) => Ok(true),
            (Pattern::Literal(lit), val) => {
                let lit_val = literal_to_value(lit);
                Ok(lit_val == *val)
            }
            (Pattern::Variable(_), _) => Ok(true), // Always match variables
        }
    }
    
    fn evaluate_binary_op(&self, left: &Value, op: BinaryOp, right: &Value) -> Result<Value, TogError> {
        match (left, op, right) {
            // Arithmetic
            (Value::Int(a), BinaryOp::Add, Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Int(a), BinaryOp::Sub, Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Int(a), BinaryOp::Mul, Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Int(a), BinaryOp::Div, Value::Int(b)) => {
                if *b == 0 {
                    Err(TogError::RuntimeError("Division by zero".to_string(), None))
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Int(a), BinaryOp::Mod, Value::Int(b)) => Ok(Value::Int(a % b)),
            
            (Value::Float(a), BinaryOp::Add, Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Float(a), BinaryOp::Sub, Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Float(a), BinaryOp::Mul, Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Float(a), BinaryOp::Div, Value::Float(b)) => {
                if *b == 0.0 {
                    Err(TogError::RuntimeError("Division by zero".to_string(), None))
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            
            // String concatenation (auto-convert numbers to strings)
            (Value::String(a), BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::String(a), BinaryOp::Add, Value::Int(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::String(a), BinaryOp::Add, Value::Float(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::Int(a), BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::Float(a), BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            
            // Comparison
            (Value::Int(a), BinaryOp::Eq, Value::Int(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), BinaryOp::Ne, Value::Int(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), BinaryOp::Lt, Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), BinaryOp::Le, Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Int(a), BinaryOp::Gt, Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), BinaryOp::Ge, Value::Int(b)) => Ok(Value::Bool(a >= b)),
            
            (Value::Bool(a), BinaryOp::And, Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
            (Value::Bool(a), BinaryOp::Or, Value::Bool(b)) => Ok(Value::Bool(*a || *b)),
            
            _ => Err(TogError::TypeError(
                format!("Invalid operation: {:?} {:?} {:?}", left, op, right),
                None
            ))
        }
    }
    
    fn evaluate_unary_op(&self, op: UnaryOp, value: &Value) -> Result<Value, TogError> {
        match (op, value) {
            (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
            (UnaryOp::Neg, Value::Int(n)) => Ok(Value::Int(-n)),
            (UnaryOp::Neg, Value::Float(n)) => Ok(Value::Float(-n)),
            _ => Err(TogError::TypeError(
                format!("Invalid unary operation: {:?} {:?}", op, value),
                None
            ))
        }
    }
}

fn literal_to_value(lit: &Literal) -> Value {
    match lit {
        Literal::Int(n) => Value::Int(*n),
        Literal::Float(n) => Value::Float(*n),
        Literal::String(s) => Value::String(s.clone()),
        Literal::Bool(b) => Value::Bool(*b),
        Literal::Array(_) => {
            // Arrays are handled in evaluate() directly
            unreachable!("Arrays should be handled in evaluate()")
        }
        Literal::None => Value::None,
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Bool(false) | Value::None => false,
        _ => true,
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Int(n) => n.to_string(),
        Value::Float(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Array(arr) => {
            let elems: Vec<String> = arr.iter().map(value_to_string).collect();
            format!("[{}]", elems.join(", "))
        }
        Value::Struct { name, fields } => {
            let mut parts: Vec<String> = Vec::new();
            for (k, v) in fields {
                parts.push(format!("{}: {}", k, value_to_string(v)));
            }
            format!("{} {{ {} }}", name, parts.join(", "))
        }
        Value::Function { name, .. } => format!("<function {}>", name),
        Value::None => "none".to_string(),
    }
}

