# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequential Simulation](#sequential-simulation)
    - [Individual Block Simulation](#individual-block-simulation)
    - [Convex Finance System Shutdown](#convex-finance-system-shutdown)

## Benchmark Results

### Sequential Simulation

|                                       | `Local_Http`           | `Ipc`                         | `ethers-reth`                  |
|:--------------------------------------|:-----------------------|:------------------------------|:------------------------------ |
| **`Blocks 14,556,786 -> 14,556,795`** | `4.10 s` (✅ **1.00x**) | `3.39 s` (✅ **1.21x faster**) | `1.73 s` (🚀 **2.37x faster**)  |

### Individual Block Simulation

|                                      | `Local_Http`              | `Ipc`                            | `ethers-reth`                     |
|:-------------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`Block: 0, TotalGas: 30,312,275`** | `802.21 ms` (✅ **1.00x**) | `670.99 ms` (✅ **1.20x faster**) | `342.05 ms` (🚀 **2.35x faster**)  |
| **`Block: 1, TotalGas: 26,490,097`** | `728.31 ms` (✅ **1.00x**) | `617.74 ms` (✅ **1.18x faster**) | `310.55 ms` (🚀 **2.35x faster**)  |
| **`Block: 2, TotalGas: 1,973,605`**  | `105.19 ms` (✅ **1.00x**) | `93.00 ms` (✅ **1.13x faster**)  | `63.51 ms` (✅ **1.66x faster**)   |
| **`Block: 3, TotalGas: 30,209,666`** | `920.93 ms` (✅ **1.00x**) | `777.15 ms` (✅ **1.19x faster**) | `380.73 ms` (🚀 **2.42x faster**)  |
| **`Block: 4, TotalGas: 30,248,521`** | `883.31 ms` (✅ **1.00x**) | `735.84 ms` (✅ **1.20x faster**) | `372.69 ms` (🚀 **2.37x faster**)  |
| **`Block: 5, TotalGas: 22,259,893`** | `700.30 ms` (✅ **1.00x**) | `581.94 ms` (✅ **1.20x faster**) | `293.28 ms` (🚀 **2.39x faster**)  |
| **`Block: 6, TotalGas: 6,195,368`**  | `219.66 ms` (✅ **1.00x**) | `189.43 ms` (✅ **1.16x faster**) | `109.54 ms` (🚀 **2.01x faster**)  |
| **`Block: 7, TotalGas: 4,067,167`**  | `180.92 ms` (✅ **1.00x**) | `155.24 ms` (✅ **1.17x faster**) | `88.21 ms` (🚀 **2.05x faster**)   |
| **`Block: 8, TotalGas: 18,144,161`** | `565.45 ms` (✅ **1.00x**) | `478.88 ms` (✅ **1.18x faster**) | `246.88 ms` (🚀 **2.29x faster**)  |
| **`Block: 9, TotalGas: 26,759,449`** | `718.40 ms` (✅ **1.00x**) | `594.82 ms` (✅ **1.21x faster**) | `299.91 ms` (🚀 **2.40x faster**)  |

### Convex Finance System Shutdown

|                       | `Local Http`              | `Ipc`                            | `ethers-reth`                     |
|:----------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`System_shutdown`** | `804.77 ms` (✅ **1.00x**) | `615.53 ms` (✅ **1.31x faster**) | `291.17 ms` (🚀 **2.76x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

