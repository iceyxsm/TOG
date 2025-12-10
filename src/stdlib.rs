// Standard library functions for TOG
use crate::interpreter::{Value, Interpreter};
use crate::error::TogError;
use std::fs;
use std::path::Path;

pub fn register_builtins(_interpreter: &mut Interpreter) {
    // Built-in functions are now dynamically called, no need to register them beforehand.
    // This function can be used in the future if we need eager registration.
}

pub fn call_builtin(name: &str, args: &[Value]) -> Result<Value, TogError> {
    match name {
        "len" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("len() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::String(s) => Ok(Value::Int(s.len() as i64)),
                Value::Array(arr) => Ok(Value::Int(arr.len() as i64)),
                _ => Err(TogError::TypeError(
                    "len() expects string or array".to_string(),
                    None
                ))
            }
        }
        "to_string" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("to_string() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            Ok(Value::String(value_to_string(&args[0])))
        }
        "range" => {
            if args.len() == 1 {
                // range(end) -> [0, 1, 2, ..., end-1]
                match &args[0] {
                    Value::Int(end) => {
                        if *end < 0 {
                            return Err(TogError::RuntimeError(
                                "range() end must be non-negative".to_string(),
                                None
                            ));
                        }
                        let arr: Vec<Value> = (0..*end).map(|i| Value::Int(i)).collect();
                        Ok(Value::Array(arr))
                    }
                    _ => Err(TogError::TypeError("range() expects Int argument".to_string(), None))
                }
            } else if args.len() == 2 {
                // range(start, end) -> [start, start+1, ..., end-1]
                match (&args[0], &args[1]) {
                    (Value::Int(start), Value::Int(end)) => {
                        if start > end {
                            return Err(TogError::RuntimeError(
                                "range() start must be <= end".to_string(),
                                None
                            ));
                        }
                        let arr: Vec<Value> = (*start..*end).map(|i| Value::Int(i)).collect();
                        Ok(Value::Array(arr))
                    }
                    _ => Err(TogError::TypeError("range() expects Int arguments".to_string(), None))
                }
            } else {
                Err(TogError::RuntimeError(
                    format!("range() expects 1 or 2 arguments, got {}", args.len()),
                    None
                ))
            }
        }
        "map" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("map() expects 2 arguments (array, function), got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::Array(_arr), Value::Function { params, .. }) => {
                    if params.len() != 1 {
                        return Err(TogError::RuntimeError(
                            "map() function must take exactly 1 argument".to_string(),
                            None
                        ));
                    }
                    Err(TogError::RuntimeError(
                        "map() requires interpreter context - use array comprehension instead".to_string(),
                        None
                    ))
                }
                _ => Err(TogError::TypeError(
                    "map() expects (array, function)".to_string(),
                    None
                ))
            }
        }
        "filter" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("filter() expects 2 arguments (array, function), got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::Array(_arr), Value::Function { params, .. }) => {
                    if params.len() != 1 {
                        return Err(TogError::RuntimeError(
                            "filter() function must take exactly 1 argument".to_string(),
                            None
                        ));
                    }
                    Err(TogError::RuntimeError(
                        "filter() requires interpreter context - use array comprehension instead".to_string(),
                        None
                    ))
                }
                _ => Err(TogError::TypeError(
                    "filter() expects (array, function)".to_string(),
                    None
                ))
            }
        }
        "reduce" => {
            if args.len() != 3 {
                return Err(TogError::RuntimeError(
                    format!("reduce() expects 3 arguments (array, initial, function), got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[2]) {
                (Value::Array(_arr), Value::Function { params, .. }) => {
                    if params.len() != 2 {
                        return Err(TogError::RuntimeError(
                            "reduce() function must take exactly 2 arguments (accumulator, element)".to_string(),
                            None
                        ));
                    }
                    Err(TogError::RuntimeError(
                        "reduce() requires interpreter context - use loop instead".to_string(),
                        None
                    ))
                }
                _ => Err(TogError::TypeError(
                    "reduce() expects (array, initial_value, function)".to_string(),
                    None
                ))
            }
        }
        // String operations
        "split" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("split() expects 2 arguments (string, delimiter), got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::String(s), Value::String(delim)) => {
                    let parts: Vec<Value> = s.split(delim)
                        .map(|part| Value::String(part.to_string()))
                        .collect();
                    Ok(Value::Array(parts))
                }
                _ => Err(TogError::TypeError("split() expects (string, string)".to_string(), None))
            }
        }
        "join" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("join() expects 2 arguments (array, delimiter), got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::Array(arr), Value::String(delim)) => {
                    let strings: Vec<String> = arr.iter()
                        .map(|v| value_to_string(v))
                        .collect();
                    Ok(Value::String(strings.join(delim)))
                }
                _ => Err(TogError::TypeError("join() expects (array, string)".to_string(), None))
            }
        }
        "contains" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("contains() expects 2 arguments, got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::String(s), Value::String(sub)) => {
                    Ok(Value::Bool(s.contains(sub)))
                }
                (Value::Array(arr), item) => {
                    Ok(Value::Bool(arr.contains(item)))
                }
                _ => Err(TogError::TypeError("contains() expects (string, string) or (array, value)".to_string(), None))
            }
        }
        "substring" => {
            if args.len() != 3 {
                return Err(TogError::RuntimeError(
                    format!("substring() expects 3 arguments (string, start, end), got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1], &args[2]) {
                (Value::String(s), Value::Int(start), Value::Int(end)) => {
                    if *start < 0 || *end < 0 || *start > *end || *end > s.len() as i64 {
                        return Err(TogError::RuntimeError(
                            format!("substring() invalid indices: start={}, end={}, len={}", start, end, s.len()),
                            None
                        ));
                    }
                    let start_usize = *start as usize;
                    let end_usize = *end as usize;
                    Ok(Value::String(s[start_usize..end_usize].to_string()))
                }
                _ => Err(TogError::TypeError("substring() expects (string, int, int)".to_string(), None))
            }
        }
        // Array operations
        "push" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("push() expects 2 arguments (array, value), got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    let mut new_arr = arr.clone();
                    new_arr.push(args[1].clone());
                    Ok(Value::Array(new_arr))
                }
                _ => Err(TogError::TypeError("push() expects array as first argument".to_string(), None))
            }
        }
        "pop" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("pop() expects 1 argument (array), got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    if arr.is_empty() {
                        return Err(TogError::RuntimeError("pop() on empty array".to_string(), None));
                    }
                    let mut new_arr = arr.clone();
                    let _popped = new_arr.pop().unwrap();
                    Ok(Value::Array(new_arr))
                }
                _ => Err(TogError::TypeError("pop() expects array".to_string(), None))
            }
        }
        "reverse" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("reverse() expects 1 argument (array), got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    let mut new_arr = arr.clone();
                    new_arr.reverse();
                    Ok(Value::Array(new_arr))
                }
                _ => Err(TogError::TypeError("reverse() expects array".to_string(), None))
            }
        }
        "append" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("append() expects 2 arguments (array, value), got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    let mut new_arr = arr.clone();
                    new_arr.push(args[1].clone());
                    Ok(Value::Array(new_arr))
                }
                _ => Err(TogError::TypeError("append() expects array as first argument".to_string(), None))
            }
        }
        // Math operations
        "min" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("min() expects 2 arguments, got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.min(b))),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
                _ => Err(TogError::TypeError("min() expects numeric arguments".to_string(), None))
            }
        }
        "max" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("max() expects 2 arguments, got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.max(b))),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
                _ => Err(TogError::TypeError("max() expects numeric arguments".to_string(), None))
            }
        }
        "abs" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("abs() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Int(n) => Ok(Value::Int(n.abs())),
                Value::Float(n) => Ok(Value::Float(n.abs())),
                _ => Err(TogError::TypeError("abs() expects numeric argument".to_string(), None))
            }
        }
        "sqrt" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("sqrt() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Int(n) => {
                    if *n < 0 {
                        return Err(TogError::RuntimeError("sqrt() of negative number".to_string(), None));
                    }
                    Ok(Value::Float((*n as f64).sqrt()))
                }
                Value::Float(n) => {
                    if *n < 0.0 {
                        return Err(TogError::RuntimeError("sqrt() of negative number".to_string(), None));
                    }
                    Ok(Value::Float(n.sqrt()))
                }
                _ => Err(TogError::TypeError("sqrt() expects numeric argument".to_string(), None))
            }
        }
        "pow" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("pow() expects 2 arguments (base, exponent), got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::Int(base), Value::Int(exp)) => {
                    Ok(Value::Int(base.pow(*exp as u32)))
                }
                (Value::Float(base), Value::Float(exp)) => {
                    Ok(Value::Float(base.powf(*exp)))
                }
                (Value::Int(base), Value::Float(exp)) => {
                    Ok(Value::Float((*base as f64).powf(*exp)))
                }
                (Value::Float(base), Value::Int(exp)) => {
                    Ok(Value::Float(base.powi(*exp as i32)))
                }
                _ => Err(TogError::TypeError("pow() expects numeric arguments".to_string(), None))
            }
        }
        // File I/O operations
        "read_file" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("read_file() expects 1 argument (filename), got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::String(filename) => {
                    match fs::read_to_string(Path::new(filename)) {
                        Ok(contents) => Ok(Value::String(contents)),
                        Err(e) => Err(TogError::IoError(format!("Failed to read file '{}': {}", filename, e)))
                    }
                }
                _ => Err(TogError::TypeError("read_file() expects string argument".to_string(), None))
            }
        }
        "write_file" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("write_file() expects 2 arguments (filename, content), got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1]) {
                (Value::String(filename), Value::String(content)) => {
                    match fs::write(Path::new(filename), content) {
                        Ok(_) => Ok(Value::None),
                        Err(e) => Err(TogError::IoError(format!("Failed to write file '{}': {}", filename, e)))
                    }
                }
                _ => Err(TogError::TypeError("write_file() expects (string, string) arguments".to_string(), None))
            }
        }
        _ => Err(TogError::RuntimeError(
            format!("Unknown builtin function: {}", name),
            None
        ))
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

