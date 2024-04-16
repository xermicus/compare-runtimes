# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Baseline](#baseline)
    - [OddProduct](#oddproduct)
    - [TriangleNumber](#trianglenumber)
    - [FibonacciRecursive](#fibonaccirecursive)
    - [FibonacciIterative](#fibonacciiterative)
    - [FibonacciBinet](#fibonaccibinet)

## Benchmark Results

### Baseline

|         | `EVM`                     | `PolkaVMInterpreter`             | `PolkaVM`                         |
|:--------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`0`** | `834.85 ns` (✅ **1.00x**) | `885.74 ns` (✅ **1.06x slower**) | `24.35 us` (❌ *29.17x slower*)    |

### OddProduct

|                 | `EVM`                     | `PolkaVMInterpreter`           | `PolkaVM`                          |
|:----------------|:--------------------------|:-------------------------------|:---------------------------------- |
| **`2000000`**   | `927.01 ms` (✅ **1.00x**) | `1.06 s` (❌ *1.15x slower*)    | `17.28 ms` (🚀 **53.65x faster**)   |
| **`4000000`**   | `1.97 s` (✅ **1.00x**)    | `2.02 s` (✅ **1.03x slower**)  | `32.40 ms` (🚀 **60.72x faster**)   |
| **`8000000`**   | `4.06 s` (✅ **1.00x**)    | `4.22 s` (✅ **1.04x slower**)  | `65.06 ms` (🚀 **62.32x faster**)   |
| **`120000000`** | `58.69 s` (✅ **1.00x**)   | `61.00 s` (✅ **1.04x slower**) | `980.21 ms` (🚀 **59.88x faster**)  |

### TriangleNumber

|                 | `EVM`                   | `PolkaVMInterpreter`           | `PolkaVM`                         |
|:----------------|:------------------------|:-------------------------------|:--------------------------------- |
| **`3000000`**   | `1.05 s` (✅ **1.00x**)  | `1.34 s` (❌ *1.28x slower*)    | `20.56 ms` (🚀 **50.85x faster**)  |
| **`6000000`**   | `2.14 s` (✅ **1.00x**)  | `2.54 s` (❌ *1.18x slower*)    | `40.89 ms` (🚀 **52.41x faster**)  |
| **`12000000`**  | `4.10 s` (✅ **1.00x**)  | `5.23 s` (❌ *1.27x slower*)    | `81.67 ms` (🚀 **50.21x faster**)  |
| **`180000000`** | `61.30 s` (✅ **1.00x**) | `72.84 s` (❌ *1.19x slower*)   | `1.22 s` (🚀 **50.14x faster**)    |

### FibonacciRecursive

|          | `EVM`                     | `PolkaVMInterpreter`             | `PolkaVM`                          |
|:---------|:--------------------------|:---------------------------------|:---------------------------------- |
| **`8`**  | `41.80 us` (✅ **1.00x**)  | `81.91 us` (❌ *1.96x slower*)    | `27.60 us` (✅ **1.51x faster**)    |
| **`12`** | `271.34 us` (✅ **1.00x**) | `532.26 us` (❌ *1.96x slower*)   | `47.42 us` (🚀 **5.72x faster**)    |
| **`16`** | `1.82 ms` (✅ **1.00x**)   | `3.76 ms` (❌ *2.07x slower*)     | `99.91 us` (🚀 **18.17x faster**)   |
| **`18`** | `4.84 ms` (✅ **1.00x**)   | `9.97 ms` (❌ *2.06x slower*)     | `201.78 us` (🚀 **24.01x faster**)  |
| **`20`** | `12.85 ms` (✅ **1.00x**)  | `26.06 ms` (❌ *2.03x slower*)    | `470.23 us` (🚀 **27.33x faster**)  |

### FibonacciIterative

|           | `EVM`                     | `PolkaVMInterpreter`             | `PolkaVM`                        |
|:----------|:--------------------------|:---------------------------------|:-------------------------------- |
| **`32`**  | `16.00 us` (✅ **1.00x**)  | `46.04 us` (❌ *2.88x slower*)    | `27.63 us` (❌ *1.73x slower*)    |
| **`64`**  | `30.62 us` (✅ **1.00x**)  | `91.79 us` (❌ *3.00x slower*)    | `40.56 us` (❌ *1.32x slower*)    |
| **`128`** | `60.25 us` (✅ **1.00x**)  | `177.04 us` (❌ *2.94x slower*)   | `35.72 us` (✅ **1.69x faster**)  |
| **`256`** | `116.88 us` (✅ **1.00x**) | `375.16 us` (❌ *3.21x slower*)   | `44.03 us` (🚀 **2.65x faster**)  |

### FibonacciBinet

|           | `EVM`                    | `PolkaVMInterpreter`             | `PolkaVM`                        |
|:----------|:-------------------------|:---------------------------------|:-------------------------------- |
| **`32`**  | `15.54 us` (✅ **1.00x**) | `107.04 us` (❌ *6.89x slower*)   | `28.06 us` (❌ *1.81x slower*)    |
| **`64`**  | `18.34 us` (✅ **1.00x**) | `128.02 us` (❌ *6.98x slower*)   | `23.59 us` (❌ *1.29x slower*)    |
| **`128`** | `20.79 us` (✅ **1.00x**) | `148.65 us` (❌ *7.15x slower*)   | `28.25 us` (❌ *1.36x slower*)    |
| **`256`** | `24.75 us` (✅ **1.00x**) | `175.84 us` (❌ *7.11x slower*)   | `28.58 us` (❌ *1.15x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

