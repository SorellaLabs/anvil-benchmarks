# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Individual Block Simulation](#individual-block-simulation)
    - [Sequential Simulation](#sequential-simulation)
    - [Convex Finance System Shutdown](#convex-finance-system-shutdown)

## Benchmark Results

### Individual Block Simulation

|                                   | `Local_Http`              | `Ipc`                            | `ethers_reth_middleware`           |
|:----------------------------------|:--------------------------|:---------------------------------|:---------------------------------- |
| **`Block_0, TotalGas: 30312275`** | `871.76 ms` (✅ **1.00x**) | `652.35 ms` (✅ **1.34x faster**) | `324.39 ms` (🚀 **2.69x faster**)   |
| **`Block_1, TotalGas: 26490097`** | `800.90 ms` (✅ **1.00x**) | `591.18 ms` (✅ **1.35x faster**) | `308.54 ms` (🚀 **2.60x faster**)   |
| **`Block_2, TotalGas: 1973605`**  | `120.78 ms` (✅ **1.00x**) | `86.20 ms` (✅ **1.40x faster**)  | `66.54 ms` (🚀 **1.82x faster**)    |
| **`Block_3, TotalGas: 30209666`** | `1.03 s` (✅ **1.00x**)    | `726.08 ms` (✅ **1.42x faster**) | `355.66 ms` (🚀 **2.91x faster**)   |
| **`Block_4, TotalGas: 30248521`** | `937.77 ms` (✅ **1.00x**) | `699.15 ms` (✅ **1.34x faster**) | `368.94 ms` (🚀 **2.54x faster**)   |
| **`Block_5, TotalGas: 22259893`** | `797.52 ms` (✅ **1.00x**) | `577.81 ms` (✅ **1.38x faster**) | `290.35 ms` (🚀 **2.75x faster**)   |
| **`Block_6, TotalGas: 6195368`**  | `243.80 ms` (✅ **1.00x**) | `174.11 ms` (✅ **1.40x faster**) | `110.93 ms` (🚀 **2.20x faster**)   |
| **`Block_7, TotalGas: 4067167`**  | `189.87 ms` (✅ **1.00x**) | `151.81 ms` (✅ **1.25x faster**) | `82.89 ms` (🚀 **2.29x faster**)    |
| **`Block_8, TotalGas: 18144161`** | `598.58 ms` (✅ **1.00x**) | `461.63 ms` (✅ **1.30x faster**) | `230.97 ms` (🚀 **2.59x faster**)   |
| **`Block_9, TotalGas: 26759449`** | `802.54 ms` (✅ **1.00x**) | `562.46 ms` (✅ **1.43x faster**) | `280.25 ms` (🚀 **2.86x faster**)   |

### Sequential Simulation

|                                   | `Local_Http`           | `Ipc`                         | `ethers_reth_middleware`           |
|:----------------------------------|:-----------------------|:------------------------------|:---------------------------------- |
| **`Blocks 14556786 -> 14556795`** | `4.57 s` (✅ **1.00x**) | `3.25 s` (✅ **1.41x faster**) | `1.69 s` (🚀 **2.71x faster**)      |

### Convex Finance System Shutdown

|                       | `Local Http`              | `Ipc`                            | `ethers-reth`                     |
|:----------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`System_shutdown`** | `880.52 ms` (✅ **1.00x**) | `644.62 ms` (✅ **1.37x faster**) | `291.73 ms` (🚀 **3.02x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

