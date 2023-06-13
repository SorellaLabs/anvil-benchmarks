# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequential Simulation](#sequential-simulation)
    - [Individual Block Simulation](#individual-block-simulation)
    - [Convex Finance System Shutdown](#convex-finance-system-shutdown)

## Benchmark Results

### Sequential Simulation

|                                       | `Local_Http`           | `Ipc`                         | `ethers_reth_middleware`           |
|:--------------------------------------|:-----------------------|:------------------------------|:---------------------------------- |
| **`Blocks 14,556,786 -> 14,556,795`** | `4.22 s` (✅ **1.00x**) | `3.31 s` (✅ **1.27x faster**) | `1.72 s` (🚀 **2.45x faster**)      |

### Individual Block Simulation

|                                      | `Local_Http`              | `Ipc`                            | `ethers_reth_middleware`           |
|:-------------------------------------|:--------------------------|:---------------------------------|:---------------------------------- |
| **`Block: 0, TotalGas: 30,312,275`** | `795.34 ms` (✅ **1.00x**) | `686.04 ms` (✅ **1.16x faster**) | `335.74 ms` (🚀 **2.37x faster**)   |
| **`Block: 1, TotalGas: 26,490,097`** | `745.08 ms` (✅ **1.00x**) | `624.51 ms` (✅ **1.19x faster**) | `326.61 ms` (🚀 **2.28x faster**)   |
| **`Block: 2, TotalGas: 1,973,605`**  | `101.47 ms` (✅ **1.00x**) | `89.23 ms` (✅ **1.14x faster**)  | `64.81 ms` (✅ **1.57x faster**)    |
| **`Block: 3, TotalGas: 30,209,666`** | `884.96 ms` (✅ **1.00x**) | `744.28 ms` (✅ **1.19x faster**) | `369.63 ms` (🚀 **2.39x faster**)   |
| **`Block: 4, TotalGas: 30,248,521`** | `909.50 ms` (✅ **1.00x**) | `748.81 ms` (✅ **1.21x faster**) | `366.58 ms` (🚀 **2.48x faster**)   |
| **`Block: 5, TotalGas: 22,259,893`** | `689.22 ms` (✅ **1.00x**) | `591.46 ms` (✅ **1.17x faster**) | `307.42 ms` (🚀 **2.24x faster**)   |
| **`Block: 6, TotalGas: 6,195,368`**  | `207.07 ms` (✅ **1.00x**) | `187.08 ms` (✅ **1.11x faster**) | `111.68 ms` (🚀 **1.85x faster**)   |
| **`Block: 7, TotalGas: 4,067,167`**  | `184.81 ms` (✅ **1.00x**) | `152.36 ms` (✅ **1.21x faster**) | `83.47 ms` (🚀 **2.21x faster**)    |
| **`Block: 8, TotalGas: 18,144,161`** | `577.01 ms` (✅ **1.00x**) | `483.47 ms` (✅ **1.19x faster**) | `238.91 ms` (🚀 **2.42x faster**)   |
| **`Block: 9, TotalGas: 26,759,449`** | `758.92 ms` (✅ **1.00x**) | `582.24 ms` (✅ **1.30x faster**) | `305.53 ms` (🚀 **2.48x faster**)   |

### Convex Finance System Shutdown

|                       | `Local Http`              | `Ipc`                            | `ethers-reth`                     |
|:----------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`System_shutdown`** | `876.58 ms` (✅ **1.00x**) | `587.53 ms` (✅ **1.49x faster**) | `276.74 ms` (🚀 **3.17x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

