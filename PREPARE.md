# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [PrepareBaseline](#preparebaseline)
    - [PrepareOddProduct](#prepareoddproduct)
    - [PrepareTriangleNumber](#preparetrianglenumber)
    - [PrepareFibonacciRecursive](#preparefibonaccirecursive)
    - [PrepareFibonacciIterative](#preparefibonacciiterative)
    - [PrepareFibonacciBinet](#preparefibonaccibinet)
    - [PrepareERC20](#prepareerc20)

## Benchmark Results

### PrepareBaseline

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                   |
|:-----------|:--------------------------|:---------------------------------|:----------------------------------|:-------------------------------------|:---------------------------------- |
| **`3779`** | `204.50 ns` (✅ **1.00x**) | `10.19 us` (❌ *49.82x slower*)   | `33.71 us` (❌ *164.85x slower*)   | `1.23 us` (❌ *5.99x slower*)         | `70.56 us` (❌ *345.01x slower*)    |

### PrepareOddProduct

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                     | `PvmInterpreterInstantiate`          | `PvmInstantiate`                  |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:-------------------------------------|:--------------------------------- |
| **`6515`** | `862.88 ns` (✅ **1.00x**) | `21.00 us` (❌ *24.34x slower*)   | `58.16 us` (❌ *67.41x slower*)   | `1.16 us` (❌ *1.34x slower*)         | `69.77 us` (❌ *80.86x slower*)    |

### PrepareTriangleNumber

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                     | `PvmInterpreterInstantiate`          | `PvmInstantiate`                  |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:-------------------------------------|:--------------------------------- |
| **`6515`** | `857.58 ns` (✅ **1.00x**) | `21.23 us` (❌ *24.76x slower*)   | `57.66 us` (❌ *67.24x slower*)   | `1.18 us` (❌ *1.37x slower*)         | `68.90 us` (❌ *80.34x slower*)    |

### PrepareFibonacciRecursive

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                   |
|:-----------|:--------------------------|:---------------------------------|:----------------------------------|:-------------------------------------|:---------------------------------- |
| **`6008`** | `424.78 ns` (✅ **1.00x**) | `18.61 us` (❌ *43.81x slower*)   | `54.87 us` (❌ *129.18x slower*)   | `1.12 us` (❌ *2.64x slower*)         | `70.19 us` (❌ *165.23x slower*)    |

### PrepareFibonacciIterative

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                   |
|:-----------|:--------------------------|:---------------------------------|:----------------------------------|:-------------------------------------|:---------------------------------- |
| **`5891`** | `440.67 ns` (✅ **1.00x**) | `19.43 us` (❌ *44.08x slower*)   | `54.79 us` (❌ *124.34x slower*)   | `1.09 us` (❌ *2.46x slower*)         | `79.48 us` (❌ *180.36x slower*)    |

### PrepareFibonacciBinet

|             | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                  |
|:------------|:--------------------------|:---------------------------------|:----------------------------------|:-------------------------------------|:--------------------------------- |
| **`10416`** | `759.06 ns` (✅ **1.00x**) | `38.20 us` (❌ *50.33x slower*)   | `91.11 us` (❌ *120.03x slower*)   | `1.24 us` (❌ *1.64x slower*)         | `68.97 us` (❌ *90.86x slower*)    |

### PrepareERC20

|             | `Evm`                   | `PvmInterpreterCompile`           | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                  |
|:------------|:------------------------|:----------------------------------|:----------------------------------|:-------------------------------------|:--------------------------------- |
| **`46845`** | `3.91 us` (✅ **1.00x**) | `157.24 us` (❌ *40.23x slower*)   | `335.99 us` (❌ *85.95x slower*)   | `1.37 us` (🚀 **2.86x faster**)       | `71.55 us` (❌ *18.30x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

