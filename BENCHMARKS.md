# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [FibonacciRecursive](#fibonaccirecursive)
    - [FibonacciIterative](#fibonacciiterative)
    - [FibonacciBinet](#fibonaccibinet)
    - [FibonacciPrepare](#fibonacciprepare)

## Benchmark Results

### FibonacciRecursive

|          | `EVM`                     | `PolkaVMInterpreter`              | `PolkaVM`                          |
|:---------|:--------------------------|:----------------------------------|:---------------------------------- |
| **`8`**  | `47.27 us` (✅ **1.00x**)  | `77.58 us` (❌ *1.64x slower*)     | `154.38 us` (❌ *3.27x slower*)     |
| **`12`** | `304.13 us` (✅ **1.00x**) | `74.72 us` (🚀 **4.07x faster**)   | `153.30 us` (🚀 **1.98x faster**)   |
| **`16`** | `2.11 ms` (✅ **1.00x**)   | `74.31 us` (🚀 **28.35x faster**)  | `156.49 us` (🚀 **13.46x faster**)  |
| **`18`** | `5.75 ms` (✅ **1.00x**)   | `79.82 us` (🚀 **72.04x faster**)  | `153.90 us` (🚀 **37.36x faster**)  |
| **`20`** | `14.46 ms` (✅ **1.00x**)  | `79.20 us` (🚀 **182.56x faster**) | `156.71 us` (🚀 **92.27x faster**)  |

### FibonacciIterative

|           | `EVM`                     | `PolkaVMInterpreter`              | `PolkaVM`                         |
|:----------|:--------------------------|:----------------------------------|:--------------------------------- |
| **`32`**  | `18.42 us` (✅ **1.00x**)  | `612.65 us` (❌ *33.25x slower*)   | `124.60 us` (❌ *6.76x slower*)    |
| **`64`**  | `35.48 us` (✅ **1.00x**)  | `613.24 us` (❌ *17.29x slower*)   | `125.54 us` (❌ *3.54x slower*)    |
| **`128`** | `69.21 us` (✅ **1.00x**)  | `612.08 us` (❌ *8.84x slower*)    | `125.97 us` (❌ *1.82x slower*)    |
| **`256`** | `138.08 us` (✅ **1.00x**) | `599.09 us` (❌ *4.34x slower*)    | `121.19 us` (✅ **1.14x faster**)  |

### FibonacciBinet

|           | `EVM`                    | `PolkaVMInterpreter`              | `PolkaVM`                         |
|:----------|:-------------------------|:----------------------------------|:--------------------------------- |
| **`32`**  | `18.81 us` (✅ **1.00x**) | `309.22 us` (❌ *16.44x slower*)   | `122.89 us` (❌ *6.53x slower*)    |
| **`64`**  | `21.88 us` (✅ **1.00x**) | `320.30 us` (❌ *14.64x slower*)   | `121.48 us` (❌ *5.55x slower*)    |
| **`128`** | `23.84 us` (✅ **1.00x**) | `310.45 us` (❌ *13.02x slower*)   | `121.52 us` (❌ *5.10x slower*)    |
| **`256`** | `27.73 us` (✅ **1.00x**) | `304.92 us` (❌ *11.00x slower*)   | `124.57 us` (❌ *4.49x slower*)    |

### FibonacciPrepare

|         | `EvmBinet`                | `EvmIterative`                  | `PolkaVMBinetInterpreter`          | `PolkaVMBinet`                     | `PolkaVMIterativeInterpreter`          | `PolkaVMIterative`                  |
|:--------|:--------------------------|:--------------------------------|:-----------------------------------|:-----------------------------------|:---------------------------------------|:----------------------------------- |
| **`0`** | `109.08 ns` (✅ **1.00x**) | `97.76 ns` (✅ **1.12x faster**) | `38.37 us` (❌ *351.71x slower*)    | `3.24 ms` (❌ *29721.17x slower*)   | `20.59 us` (❌ *188.77x slower*)        | `3.19 ms` (❌ *29220.20x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

