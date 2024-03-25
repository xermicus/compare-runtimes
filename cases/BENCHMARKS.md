# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Baseline](#baseline)
    - [FibonacciRecursive](#fibonaccirecursive)
    - [FibonacciIterative](#fibonacciiterative)
    - [FibonacciBinet](#fibonaccibinet)
    - [FibonacciPrepare](#fibonacciprepare)

## Benchmark Results

### Baseline

|        | `EVM`                     | `PolkaVMInterpreter`           | `PolkaVM`                           |
|:-------|:--------------------------|:-------------------------------|:----------------------------------- |
|        | `953.76 ns` (✅ **1.00x**) | `1.58 us` (❌ *1.65x slower*)   | `112.13 us` (❌ *117.57x slower*)    |

### FibonacciRecursive

|          | `EVM`                     | `PolkaVMInterpreter`             | `PolkaVM`                          |
|:---------|:--------------------------|:---------------------------------|:---------------------------------- |
| **`8`**  | `46.39 us` (✅ **1.00x**)  | `88.71 us` (❌ *1.91x slower*)    | `116.29 us` (❌ *2.51x slower*)     |
| **`12`** | `305.69 us` (✅ **1.00x**) | `572.87 us` (❌ *1.87x slower*)   | `126.77 us` (🚀 **2.41x faster**)   |
| **`16`** | `2.05 ms` (✅ **1.00x**)   | `3.97 ms` (❌ *1.94x slower*)     | `186.51 us` (🚀 **10.97x faster**)  |
| **`18`** | `5.43 ms` (✅ **1.00x**)   | `10.48 ms` (❌ *1.93x slower*)    | `291.29 us` (🚀 **18.63x faster**)  |
| **`20`** | `14.05 ms` (✅ **1.00x**)  | `27.08 ms` (❌ *1.93x slower*)    | `560.58 us` (🚀 **25.06x faster**)  |

### FibonacciIterative

|           | `EVM`                     | `PolkaVMInterpreter`             | `PolkaVM`                         |
|:----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`32`**  | `18.95 us` (✅ **1.00x**)  | `54.34 us` (❌ *2.87x slower*)    | `112.20 us` (❌ *5.92x slower*)    |
| **`64`**  | `35.86 us` (✅ **1.00x**)  | `107.03 us` (❌ *2.99x slower*)   | `114.50 us` (❌ *3.19x slower*)    |
| **`128`** | `70.43 us` (✅ **1.00x**)  | `203.94 us` (❌ *2.90x slower*)   | `116.10 us` (❌ *1.65x slower*)    |
| **`256`** | `140.34 us` (✅ **1.00x**) | `414.32 us` (❌ *2.95x slower*)   | `120.05 us` (✅ **1.17x faster**)  |

### FibonacciBinet

|           | `EVM`                    | `PolkaVMInterpreter`             | `PolkaVM`                         |
|:----------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`32`**  | `18.45 us` (✅ **1.00x**) | `110.43 us` (❌ *5.99x slower*)   | `116.93 us` (❌ *6.34x slower*)    |
| **`64`**  | `21.17 us` (✅ **1.00x**) | `130.15 us` (❌ *6.15x slower*)   | `118.02 us` (❌ *5.57x slower*)    |
| **`128`** | `23.55 us` (✅ **1.00x**) | `147.01 us` (❌ *6.24x slower*)   | `118.60 us` (❌ *5.04x slower*)    |
| **`256`** | `27.67 us` (✅ **1.00x**) | `172.61 us` (❌ *6.24x slower*)   | `117.15 us` (❌ *4.23x slower*)    |

### FibonacciPrepare

|         | `EvmBinet`                | `EvmIterative`                   | `PolkaVMBinetInterpreter`          | `PolkaVMBinet`                     | `PolkaVMIterativeInterpreter`          | `PolkaVMIterative`                  |
|:--------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:---------------------------------------|:----------------------------------- |
| **`0`** | `102.10 ns` (✅ **1.00x**) | `100.43 ns` (✅ **1.02x faster**) | `38.08 us` (❌ *372.94x slower*)    | `3.19 ms` (❌ *31195.07x slower*)   | `20.31 us` (❌ *198.91x slower*)        | `3.15 ms` (❌ *30850.00x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

