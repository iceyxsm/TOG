use crate::ast::*;
use crate::error::TogError;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
enum ControlFlow {
    Normal,
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &str) -> Result<Value, TogError> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }
        Err(TogError::RuntimeError(format!("Undefined variable: {}", name), None))
    }

    fn assign(&mut self, name: &str, value: Value) -> Result<(), TogError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return Ok(());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }
        Err(TogError::RuntimeError(format!("Cannot assign to undefined variable: {}", name), None))
    }

    fn remove(&mut self, name: &str) {
        self.values.remove(name);
    }
}


#[derive(Debug, Clone)]
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
    Enum {
        enum_name: String,
        variant_name: String,
        data: Option<Box<Value>>,
    },
    Function {
        name: String,
        params: Vec<Param>,
        body: Rc<Expr>,
        closure: Rc<RefCell<Environment>>,
        bound_self: Option<Box<Value>>,
    },
    None,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Struct { name: n1, fields: f1 }, Value::Struct { name: n2, fields: f2 }) => {
                n1 == n2 && f1 == f2
            }
            (Value::Enum { enum_name: e1, variant_name: v1, data: d1 }, 
             Value::Enum { enum_name: e2, variant_name: v2, data: d2 }) => {
                e1 == e2 && v1 == v2 && d1 == d2
            }
            // Functions are compared by reference/pointer, not content.
            // For simplicity here, we'll consider them unequal unless we add IDs.
            (Value::Function { .. }, Value::Function { .. }) => false,
            (Value::None, Value::None) => true,
            _ => false,
        }
    }
}


pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    struct_defs: HashMap<String, (Vec<(String, Option<Type>)>, Vec<MethodDecl>)>,
    enum_defs: HashMap<String, Vec<EnumVariant>>,
    trait_defs: HashMap<String, Vec<TraitMethod>>,
    // trait_impls: (type_name, trait_name) -> methods
    trait_impls: HashMap<(String, String), Vec<MethodDecl>>,
    // inherent_impls: type_name -> methods
    inherent_impls: HashMap<String, Vec<MethodDecl>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None))),
            struct_defs: HashMap::new(),
            enum_defs: HashMap::new(),
            trait_defs: HashMap::new(),
            trait_impls: HashMap::new(),
            inherent_impls: HashMap::new(),
        }
    }
    
    pub fn interpret(program: Program) -> Result<(), TogError> {
        let mut interpreter = Self::new();

        // Single pass execution
        for stmt in &program.statements {
            let _ = interpreter.execute_stmt(stmt)?;
        }
        
        // After all statements are executed (including function definitions),
        // find and execute the main function.
        let main_info = {
            interpreter.environment.borrow().get("main").ok().and_then(|val| {
                if let Value::Function { body, closure, .. } = val {
                    Some((body, closure))
                } else {
                    None
                }
            })
        };

        if let Some((body, closure)) = main_info {
            // Execute main in its own top-level scope.
            let old_env = Rc::clone(&interpreter.environment);
            interpreter.environment = Rc::new(RefCell::new(Environment::new(Some(closure))));
            interpreter.evaluate(&body)?;
            interpreter.environment = old_env;
        }
        
        Ok(())
    }
    
    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<(Value, ControlFlow), TogError> {
        // println!("[DEBUG] execute_stmt(): stmt: {:?}", stmt); // Removed: causes infinite recursion with closures
        match stmt {
            Stmt::Expr(expr) => {
                let val = self.evaluate(expr)?;
                Ok((val, ControlFlow::Normal))
            }
            Stmt::Let { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.environment.borrow_mut().define(name.clone(), val.clone());
                Ok((val, ControlFlow::Normal))
            }
            Stmt::Assign { name, value } => {
                let val = self.evaluate(value)?;
                self.environment.borrow_mut().assign(name, val.clone())?;
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
            Stmt::EnumDef { name, variants } => {
                self.enum_defs.insert(name.clone(), variants.clone());
                // Register enum variants as constructors in the environment
                for variant in variants {
                    let _enum_name = name.clone();
                    let _variant_name = variant.name.clone();
                    // For now, we'll handle enum construction in the evaluate phase
                    // Store enum definition for later use
                }
                Ok((Value::None, ControlFlow::Normal))
            }
            Stmt::TraitDef { name, methods } => {
                self.trait_defs.insert(name.clone(), methods.clone());
                Ok((Value::None, ControlFlow::Normal))
            }
            Stmt::ImplBlock { trait_name, type_name, methods } => {
                if let Some(trait_name) = trait_name {
                    // Trait implementation
                    self.trait_impls.insert((type_name.clone(), trait_name.clone()), methods.clone());
                } else {
                    // Inherent implementation
                    self.inherent_impls.insert(type_name.clone(), methods.clone());
                }
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
    
    fn evaluate_block(&mut self, statements: &[Stmt]) -> Result<(Value, ControlFlow), TogError> {
        let mut last_val = Value::None;
        for stmt in statements {
            let (val, control_flow) = self.execute_stmt(stmt)?;
            last_val = val;
            if control_flow != ControlFlow::Normal {
                return Ok((last_val, control_flow));
            }
        }
        Ok((last_val, ControlFlow::Normal))
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
            Expr::EnumVariant { enum_name, variant_name, data } => {
                // Validate that the enum exists
                if !self.enum_defs.contains_key(enum_name) {
                    return Err(TogError::RuntimeError(
                        format!("Unknown enum: {}", enum_name),
                        None
                    ));
                }
                
                // Evaluate the associated data if present
                let data_value = if let Some(data_expr) = data {
                    Some(Box::new(self.evaluate(data_expr)?))
                } else {
                    None
                };
                
                Ok(Value::Enum {
                    enum_name: enum_name.clone(),
                    variant_name: variant_name.clone(),
                    data: data_value,
                })
            }
            Expr::Variable(name) => {
                // Builtin functions are handled in call expressions
                self.environment.borrow().get(name)
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
                // println!("[DEBUG] evaluate_call: callee: {:?}", callee); // Removed: causes infinite recursion with closures
                let arg_values: Result<Vec<Value>, TogError> = 
                    args.iter().map(|arg| self.evaluate(arg)).collect();
                let arg_values = arg_values?;
                
                // Method call: obj.method(...) or Struct.method(...)
                if let Expr::FieldAccess { object, field: method_name } = callee.as_ref() {
                    // Check for static method call: StructName.method()
                    if let Expr::Variable(struct_name) = object.as_ref() {
                        if self.struct_defs.contains_key(struct_name) {
                            if let Some((_, methods)) = self.struct_defs.get(struct_name) {
                                if let Some(method) = methods.iter().find(|m| m.name == *method_name).cloned() {
                                    // Static method call. Execute in a new environment.
                                    let old_env = Rc::clone(&self.environment);
                                    self.environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&old_env)))));
                                    for (param, arg_value) in method.params.iter().zip(arg_values.iter()) {
                                        self.environment.borrow_mut().define(param.name.clone(), arg_value.clone());
                                    }
                                    let result = self.evaluate(&method.body);
                                    self.environment = old_env;
                                    return result;
                                }
                            }
                        }
                    }

                    let obj_val = self.evaluate(object)?;
                    if let Value::Struct { name: struct_name, .. } = obj_val.clone() {
                        if let Some((_, methods)) = self.struct_defs.get(&struct_name) {
                            if let Some(method) = methods.iter().find(|m| m.name == *method_name).cloned() {
                                
                                let old_env = Rc::clone(&self.environment);
                                // Create a new environment for the method call, enclosing the global scope.
                                self.environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&old_env)))));
                                self.environment.borrow_mut().define("self".to_string(), obj_val.clone());
                                for (param, arg_value) in method.params.iter().zip(arg_values.iter()) {
                                    self.environment.borrow_mut().define(param.name.clone(), arg_value.clone());
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
                    Value::Function { params, body, closure, bound_self, .. } => {
                        if arg_values.len() != params.len() {
                            return Err(TogError::RuntimeError(
                                format!("Function expects {} arguments, got {}", params.len(), arg_values.len()),
                                None
                            ));
                        }
                        
                        let old_env = Rc::clone(&self.environment);
                        // The new environment encloses the function's definition environment (closure).
                        self.environment = Rc::new(RefCell::new(Environment::new(Some(closure))));

                        // If a method is bound, add 'self' to the new scope
                        if let Some(self_val) = bound_self {
                            self.environment.borrow_mut().define("self".to_string(), *self_val);
                        }
                        
                        // Bind arguments to parameters in the new scope
                        for (param, arg_val) in params.iter().zip(arg_values.iter()) {
                            self.environment.borrow_mut().define(param.name.clone(), arg_val.clone());
                        }

                        let result = self.evaluate(&body);

                        self.environment = old_env;
                        return result;
                    }
                    _ => Err(TogError::TypeError(
                        "Can only call functions".to_string(),
                        None
                    ))
                }
            }
            Expr::Block(statements) => {
                let (last_val, flow) = self.evaluate_block(statements)?;
                if flow != ControlFlow::Normal {
                     return Err(TogError::RuntimeError(format!("{:?} outside of loop", flow), None));
                }
                Ok(last_val)
            }
            Expr::If { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate(condition)?;
                if is_truthy(&cond_val) {
                    self.evaluate(then_branch)
                } else if let Some(else_expr) = else_branch {
                    self.evaluate(else_expr)
                } else {
                    Ok(Value::None)
                }
            }
            Expr::While { condition, body } => {
                while is_truthy(&self.evaluate(condition)?) {
                    if let Expr::Block(statements) = body.as_ref() {
                        let (_, flow) = self.evaluate_block(statements)?;
                        match flow {
                            ControlFlow::Break => break,
                            ControlFlow::Continue => continue,
                            ControlFlow::Normal => {}
                        }
                    } else {
                        self.evaluate(body)?;
                    }
                }
                Ok(Value::None)
            }
            Expr::For { variable, iterable, body } => {
                let iterable_val = self.evaluate(iterable)?;
                let values = match iterable_val {
                    Value::Array(arr) => arr,
                    Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
                    _ => return Err(TogError::TypeError("Expected iterable in for loop".to_string(), None)),
                };

                for val in values {
                    let old_val = self.environment.borrow().get(variable).ok();
                    self.environment.borrow_mut().define(variable.clone(), val);
                    
                    if let Expr::Block(statements) = body.as_ref() {
                        let (_, flow) = self.evaluate_block(statements)?;
                        match flow {
                            ControlFlow::Break => break,
                            ControlFlow::Continue => continue,
                            ControlFlow::Normal => {}
                        }
                    } else {
                        self.evaluate(body)?;
                    }

                    if let Some(old) = old_val {
                        self.environment.borrow_mut().assign(variable, old)?;
                    } else {
                        self.environment.borrow_mut().remove(variable);
                    }
                }
                Ok(Value::None)
            }
            Expr::Match { expr, arms } => {
                let value = self.evaluate(expr)?;
                for arm in arms {
                    if self.match_pattern(&arm.pattern, &value)? {
                        // Bind pattern variables to the matched value
                        match &arm.pattern {
                            Pattern::Variable(var_name) => {
                                let old_val = self.environment.borrow().get(var_name).ok();
                                self.environment.borrow_mut().define(var_name.clone(), value.clone());
                                let result = self.evaluate(&arm.body);
                                // Restore old value if it existed
                                if let Some(old) = old_val {
                                    self.environment.borrow_mut().assign(&var_name, old)?;
                                } else {
                                    self.environment.borrow_mut().remove(var_name);
                                }
                                return result;
                            }
                            Pattern::EnumVariant { binding, .. } => {
                                // Bind the data from the enum variant
                                if let Some(binding_name) = binding {
                                    if let Value::Enum { data, .. } = &value {
                                        if let Some(data_value) = data {
                                            let old_val = self.environment.borrow().get(binding_name).ok();
                                            self.environment.borrow_mut().define(binding_name.clone(), (**data_value).clone());
                                            let result = self.evaluate(&arm.body);
                                            // Restore old value if it existed
                                            if let Some(old) = old_val {
                                                self.environment.borrow_mut().assign(&binding_name, old)?;
                                            } else {
                                                self.environment.borrow_mut().remove(binding_name);
                                            }
                                            return result;
                                        }
                                    }
                                }
                                return self.evaluate(&arm.body);
                            }
                            _ => {
                                return self.evaluate(&arm.body);
                            }
                        }
                    }
                }
                Err(TogError::RuntimeError(
                    "No matching pattern in match expression".to_string(),
                    None
                ))
            }
            Expr::Function { name, params, return_type: _, body } => {
                let func_value = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: Rc::new(*body.clone()),
                    closure: Rc::clone(&self.environment), // Capture the current environment
                    bound_self: None,
                };
                // Store function in environment
                self.environment.borrow_mut().define(name.clone(), func_value.clone());
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
                self.environment.borrow_mut().assign(name, replacement)?;
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
            (Pattern::EnumVariant { enum_name, variant_name, .. }, Value::Enum { enum_name: val_enum, variant_name: val_variant, .. }) => {
                // Match if enum name and variant name match
                Ok(enum_name == val_enum && variant_name == val_variant)
            }
            (Pattern::EnumVariant { .. }, _) => Ok(false), // Enum pattern doesn't match non-enum value
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
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Array(arr) => {
            let elems: Vec<String> = arr.iter().map(value_to_string).collect();
            format!("[{}]", elems.join(", "))
        }
        Value::Struct { name, fields } => {
            let mut parts = Vec::new();
            for (k, v) in fields {
                parts.push(format!("{}: {}", k, value_to_string(v)));
            }
            format!("{} {{ {} }}", name, parts.join(", "))
        }
        Value::Enum { enum_name, variant_name, data } => {
            if let Some(d) = data {
                format!("{}::{}({})", enum_name, variant_name, value_to_string(d))
            } else {
                format!("{}::{}", enum_name, variant_name)
            }
        }
        Value::Function { name, .. } => format!("<fn {}>", name),
        Value::None => "none".to_string(),
    }
}

