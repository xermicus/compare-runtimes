# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Baseline](#baseline)
    - [OddProduct](#oddproduct)
    - [TriangleNumber](#trianglenumber)
    - [FibonacciRecursive](#fibonaccirecursive)
    - [FibonacciIterative](#fibonacciiterative)
    - [FibonacciBinet](#fibonaccibinet)
    - [FibonacciPrepare](#fibonacciprepare)

## Benchmark Results

### Baseline

|        | `EVM`                     | `PolkaVMInterpreter`           | `PolkaVM`                           |
|:-------|:--------------------------|:-------------------------------|:----------------------------------- |
|        | `956.04 ns` (✅ **1.00x**) | `1.60 us` (❌ *1.67x slower*)   | `111.95 us` (❌ *117.10x slower*)    |

### OddProduct

|               | `EVM`                  | `PolkaVMInterpreter`          | `PolkaVM`                         |
|:--------------|:-----------------------|:------------------------------|:--------------------------------- |
| **`2000000`** | `1.19 s` (✅ **1.00x**) | `1.07 s` (✅ **1.10x faster**) | `17.18 ms` (🚀 **69.06x faster**)  |
| **`4000000`** | `2.29 s` (✅ **1.00x**) | `2.17 s` (✅ **1.06x faster**) | `33.80 ms` (🚀 **67.91x faster**)  |
| **`8000000`** | `4.58 s` (✅ **1.00x**) | `4.29 s` (✅ **1.07x faster**) | `66.70 ms` (🚀 **68.66x faster**)  |

### TriangleNumber

|                | `EVM`                  | `PolkaVMInterpreter`          | `PolkaVM`                         |
|:---------------|:-----------------------|:------------------------------|:--------------------------------- |
| **`3000000`**  | `1.25 s` (✅ **1.00x**) | `1.35 s` (✅ **1.09x slower**) | `21.90 ms` (🚀 **56.89x faster**)  |
| **`6000000`**  | `2.53 s` (✅ **1.00x**) | `2.73 s` (✅ **1.08x slower**) | `43.16 ms` (🚀 **58.68x faster**)  |
| **`12000000`** | `5.07 s` (✅ **1.00x**) | `5.41 s` (✅ **1.07x slower**) | `85.56 ms` (🚀 **59.30x faster**)  |

### FibonacciRecursive

|          | `EVM`                     | `PolkaVMInterpreter`             | `PolkaVM`                          |
|:---------|:--------------------------|:---------------------------------|:---------------------------------- |
| **`8`**  | `46.34 us` (✅ **1.00x**)  | `85.42 us` (❌ *1.84x slower*)    | `116.70 us` (❌ *2.52x slower*)     |
| **`12`** | `303.89 us` (✅ **1.00x**) | `561.43 us` (❌ *1.85x slower*)   | `126.53 us` (🚀 **2.40x faster**)   |
| **`16`** | `2.13 ms` (✅ **1.00x**)   | `3.94 ms` (❌ *1.85x slower*)     | `182.31 us` (🚀 **11.66x faster**)  |
| **`18`** | `5.38 ms` (✅ **1.00x**)   | `10.27 ms` (❌ *1.91x slower*)    | `294.24 us` (🚀 **18.29x faster**)  |
| **`20`** | `14.61 ms` (✅ **1.00x**)  | `26.91 ms` (❌ *1.84x slower*)    | `568.63 us` (🚀 **25.70x faster**)  |

### FibonacciIterative

|           | `EVM`                     | `PolkaVMInterpreter`             | `PolkaVM`                         |
|:----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`32`**  | `18.95 us` (✅ **1.00x**)  | `52.79 us` (❌ *2.79x slower*)    | `114.67 us` (❌ *6.05x slower*)    |
| **`64`**  | `35.61 us` (✅ **1.00x**)  | `103.89 us` (❌ *2.92x slower*)   | `114.46 us` (❌ *3.21x slower*)    |
| **`128`** | `69.14 us` (✅ **1.00x**)  | `210.48 us` (❌ *3.04x slower*)   | `115.86 us` (❌ *1.68x slower*)    |
| **`256`** | `138.02 us` (✅ **1.00x**) | `427.26 us` (❌ *3.10x slower*)   | `118.55 us` (✅ **1.16x faster**)  |

### FibonacciBinet

|           | `EVM`                    | `PolkaVMInterpreter`             | `PolkaVM`                         |
|:----------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`32`**  | `18.39 us` (✅ **1.00x**) | `115.78 us` (❌ *6.30x slower*)   | `118.17 us` (❌ *6.43x slower*)    |
| **`64`**  | `21.18 us` (✅ **1.00x**) | `137.75 us` (❌ *6.50x slower*)   | `118.77 us` (❌ *5.61x slower*)    |
| **`128`** | `23.69 us` (✅ **1.00x**) | `158.17 us` (❌ *6.68x slower*)   | `117.99 us` (❌ *4.98x slower*)    |
| **`256`** | `27.67 us` (✅ **1.00x**) | `181.30 us` (❌ *6.55x slower*)   | `118.02 us` (❌ *4.27x slower*)    |

### FibonacciPrepare

|         | `EvmBinet`                | `EvmIterative`                  | `PolkaVMBinetInterpreter`          | `PolkaVMBinet`                     | `PolkaVMIterativeInterpreter`          | `PolkaVMIterative`                  |
|:--------|:--------------------------|:--------------------------------|:-----------------------------------|:-----------------------------------|:---------------------------------------|:----------------------------------- |
| **`0`** | `102.09 ns` (✅ **1.00x**) | `98.52 ns` (✅ **1.04x faster**) | `39.60 us` (❌ *387.91x slower*)    | `3.22 ms` (❌ *31555.11x slower*)   | `21.32 us` (❌ *208.79x slower*)        | `3.17 ms` (❌ *31082.99x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

