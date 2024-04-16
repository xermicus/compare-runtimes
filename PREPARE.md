# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Baseline](#baseline)
    - [OddProduct](#oddproduct)
    - [TriangleNumber](#trianglenumber)
    - [FibonacciRecursive](#fibonaccirecursive)
    - [FibonacciIterative](#fibonacciiterative)
    - [FibonacciBinet](#fibonaccibinet)
    - [ERC20](#erc20)

## Benchmark Results

### Baseline

|         | `PrepareEvm`              | `PreparePolkaVMInterpreter`          | `PreparePolkaVM`                    |
|:--------|:--------------------------|:-------------------------------------|:----------------------------------- |
| **`0`** | `202.55 ns` (✅ **1.00x**) | `11.74 us` (❌ *57.95x slower*)       | `2.95 ms` (❌ *14582.73x slower*)    |

### OddProduct

|         | `PrepareEvm`              | `PreparePolkaVMInterpreter`          | `PreparePolkaVM`                   |
|:--------|:--------------------------|:-------------------------------------|:---------------------------------- |
| **`0`** | `886.29 ns` (✅ **1.00x**) | `21.79 us` (❌ *24.59x slower*)       | `2.98 ms` (❌ *3358.41x slower*)    |

### TriangleNumber

|         | `PrepareEvm`              | `PreparePolkaVMInterpreter`          | `PreparePolkaVM`                   |
|:--------|:--------------------------|:-------------------------------------|:---------------------------------- |
| **`0`** | `865.94 ns` (✅ **1.00x**) | `22.68 us` (❌ *26.19x slower*)       | `2.98 ms` (❌ *3437.30x slower*)    |

### FibonacciRecursive

|         | `PrepareEvm`              | `PreparePolkaVMInterpreter`          | `PreparePolkaVM`                   |
|:--------|:--------------------------|:-------------------------------------|:---------------------------------- |
| **`0`** | `437.36 ns` (✅ **1.00x**) | `20.99 us` (❌ *48.00x slower*)       | `2.98 ms` (❌ *6810.11x slower*)    |

### FibonacciIterative

|         | `PrepareEvm`              | `PreparePolkaVMInterpreter`          | `PreparePolkaVM`                   |
|:--------|:--------------------------|:-------------------------------------|:---------------------------------- |
| **`0`** | `413.22 ns` (✅ **1.00x**) | `20.26 us` (❌ *49.03x slower*)       | `2.98 ms` (❌ *7218.55x slower*)    |

### FibonacciBinet

|         | `PrepareEvm`              | `PreparePolkaVMInterpreter`          | `PreparePolkaVM`                   |
|:--------|:--------------------------|:-------------------------------------|:---------------------------------- |
| **`0`** | `702.48 ns` (✅ **1.00x**) | `38.33 us` (❌ *54.56x slower*)       | `3.01 ms` (❌ *4283.89x slower*)    |

### ERC20

|         | `PrepareEvm`            | `PreparePolkaVMInterpreter`          | `PreparePolkaVM`                  |
|:--------|:------------------------|:-------------------------------------|:--------------------------------- |
| **`0`** | `3.91 us` (✅ **1.00x**) | `163.36 us` (❌ *41.77x slower*)      | `3.31 ms` (❌ *845.39x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

