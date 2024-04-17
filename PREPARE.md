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
| **`3779`** | `206.20 ns` (✅ **1.00x**) | `10.15 us` (❌ *49.23x slower*)   | `32.64 us` (❌ *158.31x slower*)   | `1.16 us` (❌ *5.63x slower*)         | `65.66 us` (❌ *318.45x slower*)    |

### PrepareOddProduct

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                     | `PvmInterpreterInstantiate`          | `PvmInstantiate`                  |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:-------------------------------------|:--------------------------------- |
| **`6515`** | `898.86 ns` (✅ **1.00x**) | `20.19 us` (❌ *22.46x slower*)   | `55.49 us` (❌ *61.74x slower*)   | `1.22 us` (❌ *1.36x slower*)         | `70.10 us` (❌ *77.99x slower*)    |

### PrepareTriangleNumber

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                     | `PvmInterpreterInstantiate`          | `PvmInstantiate`                  |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:-------------------------------------|:--------------------------------- |
| **`6515`** | `898.04 ns` (✅ **1.00x**) | `21.20 us` (❌ *23.61x slower*)   | `56.33 us` (❌ *62.72x slower*)   | `1.32 us` (❌ *1.47x slower*)         | `69.15 us` (❌ *77.00x slower*)    |

### PrepareFibonacciRecursive

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                   |
|:-----------|:--------------------------|:---------------------------------|:----------------------------------|:-------------------------------------|:---------------------------------- |
| **`6008`** | `422.14 ns` (✅ **1.00x**) | `18.87 us` (❌ *44.69x slower*)   | `55.72 us` (❌ *132.00x slower*)   | `1.17 us` (❌ *2.78x slower*)         | `67.32 us` (❌ *159.47x slower*)    |

### PrepareFibonacciIterative

|            | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                   |
|:-----------|:--------------------------|:---------------------------------|:----------------------------------|:-------------------------------------|:---------------------------------- |
| **`5891`** | `438.56 ns` (✅ **1.00x**) | `19.10 us` (❌ *43.54x slower*)   | `53.15 us` (❌ *121.18x slower*)   | `1.11 us` (❌ *2.53x slower*)         | `70.34 us` (❌ *160.38x slower*)    |

### PrepareFibonacciBinet

|             | `Evm`                     | `PvmInterpreterCompile`          | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                   |
|:------------|:--------------------------|:---------------------------------|:----------------------------------|:-------------------------------------|:---------------------------------- |
| **`10416`** | `680.91 ns` (✅ **1.00x**) | `36.08 us` (❌ *52.99x slower*)   | `88.45 us` (❌ *129.89x slower*)   | `1.23 us` (❌ *1.80x slower*)         | `68.28 us` (❌ *100.27x slower*)    |

### PrepareERC20

|             | `Evm`                   | `PvmInterpreterCompile`           | `PvmCompile`                      | `PvmInterpreterInstantiate`          | `PvmInstantiate`                  |
|:------------|:------------------------|:----------------------------------|:----------------------------------|:-------------------------------------|:--------------------------------- |
| **`46845`** | `3.88 us` (✅ **1.00x**) | `157.74 us` (❌ *40.65x slower*)   | `335.76 us` (❌ *86.52x slower*)   | `1.31 us` (🚀 **2.96x faster**)       | `69.42 us` (❌ *17.89x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

