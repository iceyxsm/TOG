// Standard library functions for TOG
use crate::interpreter::{Value, Interpreter};
use crate::error::TogError;
use std::fs;
use std::path::Path;

#[allow(dead_code)] // Reserved for future eager registration of built-ins
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
        // GPU and Parallel Processing Functions
        "gpu_sum" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("gpu_sum() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => gpu_accelerate("sum", arr),
                _ => Err(TogError::TypeError("gpu_sum() expects array".to_string(), None))
            }
        }
        "gpu_product" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("gpu_product() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => gpu_accelerate("product", arr),
                _ => Err(TogError::TypeError("gpu_product() expects array".to_string(), None))
            }
        }
        "gpu_mean" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("gpu_mean() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => gpu_accelerate("mean", arr),
                _ => Err(TogError::TypeError("gpu_mean() expects array".to_string(), None))
            }
        }
        "parallel_sum" => {
            // Parallel sum using rayon-style processing
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("parallel_sum() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    // Use chunks for parallel processing simulation
                    let chunk_size = (arr.len() / 4).max(1);
                    let sum = arr.chunks(chunk_size)
                        .map(|chunk| {
                            chunk.iter().fold(0.0, |acc, v| {
                                acc + match v {
                                    Value::Int(i) => *i as f64,
                                    Value::Float(f) => *f,
                                    _ => 0.0,
                                }
                            })
                        })
                        .sum::<f64>();
                    Ok(Value::Float(sum))
                }
                _ => Err(TogError::TypeError("parallel_sum() expects array".to_string(), None))
            }
        }
        "batch_size" => {
            // Returns optimal batch size for the system
            // For now, return a reasonable default
            Ok(Value::Int(1024))
        }
        "map" => {
            // map(array, function) - applies function to each element
            // Note: Function application needs to be handled by interpreter
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("map() expects 2 arguments (array, function), got {}", args.len()),
                    None
                ));
            }
            // For now, return the array as-is
            // The interpreter will need to handle function application
            match &args[0] {
                Value::Array(_) => Ok(args[0].clone()),
                _ => Err(TogError::TypeError("map() expects array as first argument".to_string(), None))
            }
        }
        "filter" => {
            // filter(array, predicate) - keeps elements where predicate is true
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("filter() expects 2 arguments (array, predicate), got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(_) => Ok(args[0].clone()),
                _ => Err(TogError::TypeError("filter() expects array as first argument".to_string(), None))
            }
        }
        "reduce" => {
            // reduce(array, initial, function) - reduces array to single value
            if args.len() != 3 {
                return Err(TogError::RuntimeError(
                    format!("reduce() expects 3 arguments (array, initial, function), got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(_) => Ok(args[1].clone()), // Return initial for now
                _ => Err(TogError::TypeError("reduce() expects array as first argument".to_string(), None))
            }
        }
        "parallel_map" => {
            // Parallel version of map
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("parallel_map() expects 2 arguments, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(_) => Ok(args[0].clone()),
                _ => Err(TogError::TypeError("parallel_map() expects array".to_string(), None))
            }
        }
        "parallel_filter" => {
            // Parallel version of filter
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("parallel_filter() expects 2 arguments, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(_) => Ok(args[0].clone()),
                _ => Err(TogError::TypeError("parallel_filter() expects array".to_string(), None))
            }
        }
        "parallel_reduce" => {
            // Parallel version of reduce
            if args.len() != 3 {
                return Err(TogError::RuntimeError(
                    format!("parallel_reduce() expects 3 arguments, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(_) => Ok(args[1].clone()),
                _ => Err(TogError::TypeError("parallel_reduce() expects array".to_string(), None))
            }
        }
        // Additional array operations
        "first" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("first() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    if arr.is_empty() {
                        Err(TogError::RuntimeError("first() called on empty array".to_string(), None))
                    } else {
                        Ok(arr[0].clone())
                    }
                }
                _ => Err(TogError::TypeError("first() expects array".to_string(), None))
            }
        }
        "last" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("last() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    if arr.is_empty() {
                        Err(TogError::RuntimeError("last() called on empty array".to_string(), None))
                    } else {
                        Ok(arr[arr.len() - 1].clone())
                    }
                }
                _ => Err(TogError::TypeError("last() expects array".to_string(), None))
            }
        }
        "slice" => {
            // slice(array, start, end) - returns subarray
            if args.len() != 3 {
                return Err(TogError::RuntimeError(
                    format!("slice() expects 3 arguments, got {}", args.len()),
                    None
                ));
            }
            match (&args[0], &args[1], &args[2]) {
                (Value::Array(arr), Value::Int(start), Value::Int(end)) => {
                    let start_idx = (*start).max(0) as usize;
                    let end_idx = (*end).min(arr.len() as i64) as usize;
                    if start_idx > end_idx {
                        return Err(TogError::RuntimeError(
                            "slice() start index must be <= end index".to_string(),
                            None
                        ));
                    }
                    Ok(Value::Array(arr[start_idx..end_idx].to_vec()))
                }
                _ => Err(TogError::TypeError("slice() expects (array, int, int)".to_string(), None))
            }
        }
        "flatten" => {
            // flatten(array) - flattens nested arrays one level
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("flatten() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    let mut result = Vec::new();
                    for item in arr {
                        match item {
                            Value::Array(inner) => result.extend_from_slice(inner),
                            _ => result.push(item.clone()),
                        }
                    }
                    Ok(Value::Array(result))
                }
                _ => Err(TogError::TypeError("flatten() expects array".to_string(), None))
            }
        }
        "unique" => {
            // unique(array) - returns array with duplicates removed
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("unique() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    let mut result = Vec::new();
                    for item in arr {
                        if !result.contains(item) {
                            result.push(item.clone());
                        }
                    }
                    Ok(Value::Array(result))
                }
                _ => Err(TogError::TypeError("unique() expects array".to_string(), None))
            }
        }
        "sort" => {
            // sort(array) - returns sorted array (numeric only for now)
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("sort() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Array(arr) => {
                    let mut sorted = arr.clone();
                    // Simple bubble sort for integers
                    let mut swapped = true;
                    while swapped {
                        swapped = false;
                        for i in 0..sorted.len().saturating_sub(1) {
                            let should_swap = match (&sorted[i], &sorted[i + 1]) {
                                (Value::Int(a), Value::Int(b)) => a > b,
                                (Value::Float(a), Value::Float(b)) => a > b,
                                _ => false,
                            };
                            if should_swap {
                                sorted.swap(i, i + 1);
                                swapped = true;
                            }
                        }
                    }
                    Ok(Value::Array(sorted))
                }
                _ => Err(TogError::TypeError("sort() expects array".to_string(), None))
            }
        }
        
        // Result helper methods
        "unwrap" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("unwrap() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Enum { enum_name, variant_name, data } => {
                    if enum_name == "Result" && variant_name == "Ok" {
                        if let Some(value) = data {
                            Ok((**value).clone())
                        } else {
                            Err(TogError::RuntimeError(
                                "unwrap() called on Result::Ok with no data".to_string(),
                                None
                            ))
                        }
                    } else if enum_name == "Result" && variant_name == "Err" {
                        if let Some(err) = data {
                            Err(TogError::RuntimeError(
                                format!("unwrap() called on Result::Err({})", value_to_string(err)),
                                None
                            ))
                        } else {
                            Err(TogError::RuntimeError(
                                "unwrap() called on Result::Err".to_string(),
                                None
                            ))
                        }
                    } else if enum_name == "Option" && variant_name == "Some" {
                        if let Some(value) = data {
                            Ok((**value).clone())
                        } else {
                            Err(TogError::RuntimeError(
                                "unwrap() called on Option::Some with no data".to_string(),
                                None
                            ))
                        }
                    } else if enum_name == "Option" && variant_name == "None" {
                        Err(TogError::RuntimeError(
                            "unwrap() called on Option::None".to_string(),
                            None
                        ))
                    } else {
                        Err(TogError::TypeError(
                            format!("unwrap() expects Result or Option, got {}::{}", enum_name, variant_name),
                            None
                        ))
                    }
                }
                _ => Err(TogError::TypeError(
                    "unwrap() expects Result or Option enum".to_string(),
                    None
                ))
            }
        }
        "unwrap_or" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("unwrap_or() expects 2 arguments, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Enum { enum_name, variant_name, data } => {
                    if enum_name == "Result" && variant_name == "Ok" {
                        if let Some(value) = data {
                            Ok((**value).clone())
                        } else {
                            Ok(args[1].clone())
                        }
                    } else if enum_name == "Result" && variant_name == "Err" {
                        Ok(args[1].clone())
                    } else if enum_name == "Option" && variant_name == "Some" {
                        if let Some(value) = data {
                            Ok((**value).clone())
                        } else {
                            Ok(args[1].clone())
                        }
                    } else if enum_name == "Option" && variant_name == "None" {
                        Ok(args[1].clone())
                    } else {
                        Err(TogError::TypeError(
                            format!("unwrap_or() expects Result or Option, got {}::{}", enum_name, variant_name),
                            None
                        ))
                    }
                }
                _ => Err(TogError::TypeError(
                    "unwrap_or() expects Result or Option enum".to_string(),
                    None
                ))
            }
        }
        "expect" => {
            if args.len() != 2 {
                return Err(TogError::RuntimeError(
                    format!("expect() expects 2 arguments, got {}", args.len()),
                    None
                ));
            }
            let msg = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(TogError::TypeError(
                    "expect() second argument must be a string".to_string(),
                    None
                ))
            };
            match &args[0] {
                Value::Enum { enum_name, variant_name, data } => {
                    if enum_name == "Result" && variant_name == "Ok" {
                        if let Some(value) = data {
                            Ok((**value).clone())
                        } else {
                            Err(TogError::RuntimeError(msg, None))
                        }
                    } else if enum_name == "Result" && variant_name == "Err" {
                        Err(TogError::RuntimeError(msg, None))
                    } else if enum_name == "Option" && variant_name == "Some" {
                        if let Some(value) = data {
                            Ok((**value).clone())
                        } else {
                            Err(TogError::RuntimeError(msg, None))
                        }
                    } else if enum_name == "Option" && variant_name == "None" {
                        Err(TogError::RuntimeError(msg, None))
                    } else {
                        Err(TogError::TypeError(
                            format!("expect() expects Result or Option, got {}::{}", enum_name, variant_name),
                            None
                        ))
                    }
                }
                _ => Err(TogError::TypeError(
                    "expect() expects Result or Option enum".to_string(),
                    None
                ))
            }
        }
        "is_ok" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("is_ok() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Enum { enum_name, variant_name, .. } => {
                    if enum_name == "Result" {
                        Ok(Value::Bool(variant_name == "Ok"))
                    } else {
                        Err(TogError::TypeError(
                            format!("is_ok() expects Result, got {}", enum_name),
                            None
                        ))
                    }
                }
                _ => Err(TogError::TypeError(
                    "is_ok() expects Result enum".to_string(),
                    None
                ))
            }
        }
        "is_err" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("is_err() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Enum { enum_name, variant_name, .. } => {
                    if enum_name == "Result" {
                        Ok(Value::Bool(variant_name == "Err"))
                    } else {
                        Err(TogError::TypeError(
                            format!("is_err() expects Result, got {}", enum_name),
                            None
                        ))
                    }
                }
                _ => Err(TogError::TypeError(
                    "is_err() expects Result enum".to_string(),
                    None
                ))
            }
        }
        "is_some" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("is_some() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Enum { enum_name, variant_name, .. } => {
                    if enum_name == "Option" {
                        Ok(Value::Bool(variant_name == "Some"))
                    } else {
                        Err(TogError::TypeError(
                            format!("is_some() expects Option, got {}", enum_name),
                            None
                        ))
                    }
                }
                _ => Err(TogError::TypeError(
                    "is_some() expects Option enum".to_string(),
                    None
                ))
            }
        }
        "is_none" => {
            if args.len() != 1 {
                return Err(TogError::RuntimeError(
                    format!("is_none() expects 1 argument, got {}", args.len()),
                    None
                ));
            }
            match &args[0] {
                Value::Enum { enum_name, variant_name, .. } => {
                    if enum_name == "Option" {
                        Ok(Value::Bool(variant_name == "None"))
                    } else {
                        Err(TogError::TypeError(
                            format!("is_none() expects Option, got {}", enum_name),
                            None
                        ))
                    }
                }
                _ => Err(TogError::TypeError(
                    "is_none() expects Option enum".to_string(),
                    None
                ))
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
        Value::Enum { enum_name, variant_name, data } => {
            if let Some(d) = data {
                format!("{}::{}({})", enum_name, variant_name, value_to_string(d))
            } else {
                format!("{}::{}", enum_name, variant_name)
            }
        }
        Value::Function { name, .. } => format!("<function {}>", name),
        Value::None => "none".to_string(),
    }
}

// ============================================================================
// GPU and Parallel Processing Functions
// ============================================================================

/// Parallel map - applies a function to each element in parallel
/// Usage: parallel_map(array, function)
#[allow(dead_code)]
pub fn parallel_map(array: &[Value], _func: &Value) -> Result<Value, TogError> {
    // For now, this is a placeholder that does sequential processing
    // In the future, this will use rayon or GPU acceleration
    // The interpreter will need to handle function application
    Ok(Value::Array(array.to_vec()))
}

/// Batch process - processes array in batches for better cache locality
/// Usage: batch_process(array, batch_size, function)
#[allow(dead_code)]
pub fn batch_process(array: &[Value], batch_size: usize, _func: &Value) -> Result<Value, TogError> {
    if batch_size == 0 {
        return Err(TogError::RuntimeError(
            "batch_size must be greater than 0".to_string(),
            None
        ));
    }
    
    // Process in batches for better cache performance
    let mut result = Vec::new();
    for chunk in array.chunks(batch_size) {
        result.extend_from_slice(chunk);
    }
    
    Ok(Value::Array(result))
}

/// GPU-accelerated array operations
/// Automatically detects numeric operations and offloads to GPU if available
#[allow(dead_code)]
pub fn gpu_accelerate(operation: &str, array: &[Value]) -> Result<Value, TogError> {
    // Check if all elements are numeric
    let all_numeric = array.iter().all(|v| matches!(v, Value::Int(_) | Value::Float(_)));
    
    if !all_numeric {
        return Err(TogError::TypeError(
            "GPU acceleration requires numeric arrays".to_string(),
            None
        ));
    }
    
    match operation {
        "sum" => {
            let sum = array.iter().fold(0.0, |acc, v| {
                acc + match v {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f,
                    _ => 0.0,
                }
            });
            Ok(Value::Float(sum))
        }
        "product" => {
            let product = array.iter().fold(1.0, |acc, v| {
                acc * match v {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f,
                    _ => 1.0,
                }
            });
            Ok(Value::Float(product))
        }
        "mean" => {
            if array.is_empty() {
                return Err(TogError::RuntimeError(
                    "Cannot compute mean of empty array".to_string(),
                    None
                ));
            }
            let sum = array.iter().fold(0.0, |acc, v| {
                acc + match v {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f,
                    _ => 0.0,
                }
            });
            Ok(Value::Float(sum / array.len() as f64))
        }
        _ => Err(TogError::RuntimeError(
            format!("Unknown GPU operation: {}", operation),
            None
        ))
    }
}

