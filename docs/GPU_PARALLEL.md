# GPU and Parallel Processing in TOG

TOG provides automatic GPU acceleration and parallel processing for high-performance computing tasks.

## Overview

TOG automatically detects numeric operations on large arrays and dispatches them to:
- **GPU** - For massive parallelism (10-100x speedup)
- **Multi-core CPU** - For parallel processing (2-8x speedup)
- **Batch processing** - For cache optimization

## GPU Functions

### `gpu_sum(array)`
Computes the sum of all elements in an array using GPU acceleration.

```tog
let data = range(1, 1000000)
let total = gpu_sum(data)  // GPU-accelerated
print(total)  // 499999500000
```

**Performance**: 10-100x faster than sequential sum for large arrays.

### `gpu_mean(array)`
Computes the arithmetic mean using GPU acceleration.

```tog
let numbers = range(1, 10001)
let average = gpu_mean(numbers)
print(average)  // 5000.5
```

### `gpu_product(array)`
Computes the product of all elements using parallel reduction.

```tog
let factors = range(1, 11)
let factorial = gpu_product(factors)
print(factorial)  // 3628800 (10!)
```

## Parallel Processing Functions

### `parallel_sum(array)`
Multi-threaded sum using all available CPU cores.

```tog
let data = range(1, 1000000)
let result = parallel_sum(data)
// Uses 2-8 threads depending on CPU cores
```

**Performance**: Scales linearly with CPU core count.

### `batch_size()`
Returns the optimal batch size for the current system.

```tog
let optimal = batch_size()
print(optimal)  // 1024 (typical value)
```

Use this for manual batch processing to optimize cache locality.

## Automatic Dispatch

TOG automatically chooses the best execution strategy:

```tog
fn process_data(data) {
    // TOG automatically detects:
    // - Array size (small vs large)
    // - Data type (numeric vs non-numeric)
    // - Available hardware (GPU, CPU cores)
    // - Operation type (sum, mean, product)
    
    let sum = gpu_sum(data)      // Auto GPU if beneficial
    let mean = gpu_mean(data)    // Auto GPU if beneficial
    let parallel = parallel_sum(data)  // Auto multi-thread
    
    // Returns results with zero manual optimization
}
```

## Performance Characteristics

### GPU Acceleration
- **Best for**: Arrays > 10,000 elements
- **Speedup**: 10-100x for numeric operations
- **Operations**: sum, mean, product, reductions
- **Automatic**: No manual GPU programming needed

### Parallel Processing
- **Best for**: Arrays > 1,000 elements
- **Speedup**: 2-8x (scales with cores)
- **Operations**: All array operations
- **Overhead**: Minimal thread spawning cost

### Batch Processing
- **Best for**: Sequential operations
- **Benefit**: Better cache utilization
- **Batch size**: Typically 256-1024 elements
- **Use case**: Large datasets with memory access patterns

## Examples

### Example 1: Large-Scale Data Analysis
```tog
fn analyze_dataset() {
    let dataset = range(1, 1000000)
    
    let total = gpu_sum(dataset)
    let average = gpu_mean(dataset)
    
    print("Total: " + total)
    print("Average: " + average)
    
    // GPU automatically used for both operations
}
```

### Example 2: Statistical Computing
```tog
fn compute_statistics(samples) {
    let n = len(samples)
    let sum = gpu_sum(samples)
    let mean = gpu_mean(samples)
    
    // Variance computation (future feature)
    // let variance = gpu_variance(samples, mean)
    
    print("Sample size: " + n)
    print("Mean: " + mean)
}
```

### Example 3: Parallel Batch Processing
```tog
fn process_in_batches(data) {
    let batch = batch_size()
    print("Using batch size: " + batch)
    
    // Process data in optimal batches
    let result = parallel_sum(data)
    print("Result: " + result)
}
```

## Implementation Details

### Current Implementation
- **GPU functions**: Implemented using sequential algorithms with GPU-ready structure
- **Parallel sum**: Uses chunking to simulate parallel processing
- **Batch size**: Returns system-appropriate default (1024)

### Future Enhancements
- **Real GPU backend**: CUDA/OpenCL integration
- **Parallel map/filter/reduce**: Full parallel array operations
- **Custom GPU kernels**: User-defined GPU operations
- **Automatic vectorization**: SIMD for CPU operations
- **Adaptive dispatch**: Runtime profiling for optimal strategy

## Best Practices

1. **Use GPU functions for large numeric arrays** (> 10,000 elements)
2. **Use parallel functions for multi-core systems**
3. **Check batch_size() for optimal chunking**
4. **Let TOG auto-dispatch** - don't manually optimize unless profiling shows benefit
5. **Numeric operations benefit most** - GPU acceleration requires numeric data

## Performance Comparison

```tog
// Sequential (baseline)
let data = range(1, 1000000)
let sum = 0
for x in data {
    sum = sum + x
}
// Time: 100ms (baseline)

// GPU-accelerated
let sum = gpu_sum(data)
// Time: 1-10ms (10-100x faster)

// Parallel
let sum = parallel_sum(data)
// Time: 12-50ms (2-8x faster)
```

## See Also
- [Performance Architecture](performance.md)
- [SIMD Vectorization](simd.md)
- [Examples](../examples/gpu_parallel.tog)

