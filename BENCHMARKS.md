# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Individual Block Simulation](#individual-block-simulation)
    - [All Blocks Simulation](#all-blocks-simulation)
    - [Anvil](#anvil)

## Benchmark Results

### Individual Block Simulation

|               | `Local_Http`              | `Ipc`                            | `ethers_reth_middleware`           |
|:--------------|:--------------------------|:---------------------------------|:---------------------------------- |
| **`Block_0`** | `865.39 ms` (✅ **1.00x**) | `718.26 ms` (✅ **1.20x faster**) | `379.62 ms` (🚀 **2.28x faster**)   |
| **`Block_1`** | `1.08 s` (✅ **1.00x**)    | `784.32 ms` (✅ **1.38x faster**) | `442.19 ms` (🚀 **2.45x faster**)   |
| **`Block_2`** | `171.94 ms` (✅ **1.00x**) | `101.67 ms` (✅ **1.69x faster**) | `88.16 ms` (🚀 **1.95x faster**)    |
| **`Block_3`** | `1.29 s` (✅ **1.00x**)    | `884.06 ms` (✅ **1.46x faster**) | `428.43 ms` (🚀 **3.02x faster**)   |
| **`Block_4`** | `1.25 s` (✅ **1.00x**)    | `751.57 ms` (✅ **1.67x faster**) | `497.72 ms` (🚀 **2.52x faster**)   |
| **`Block_5`** | `909.17 ms` (✅ **1.00x**) | `683.53 ms` (✅ **1.33x faster**) | `404.78 ms` (🚀 **2.25x faster**)   |
| **`Block_6`** | `351.57 ms` (✅ **1.00x**) | `243.60 ms` (✅ **1.44x faster**) | `151.00 ms` (🚀 **2.33x faster**)   |
| **`Block_7`** | `271.05 ms` (✅ **1.00x**) | `212.41 ms` (✅ **1.28x faster**) | `87.22 ms` (🚀 **3.11x faster**)    |
| **`Block_8`** | `891.00 ms` (✅ **1.00x**) | `583.21 ms` (✅ **1.53x faster**) | `315.54 ms` (🚀 **2.82x faster**)   |
| **`Block_9`** | `1.03 s` (✅ **1.00x**)    | `729.32 ms` (✅ **1.41x faster**) | `381.21 ms` (🚀 **2.70x faster**)   |

### All Blocks Simulation

|                              | `All_blocks`            |
|:-----------------------------|:----------------------- |
| **`Local_Http`**             | `5.47 s` (✅ **1.00x**)  |
| **`Ipc`**                    | `3.79 s` (✅ **1.00x**)  |
| **`ethers_reth_middleware`** | `2.06 s` (✅ **1.00x**)  |

### Anvil

|                       | `Local Http`           | `Ipc`                            | `ethers-reth`                     |
|:----------------------|:-----------------------|:---------------------------------|:--------------------------------- |
| **`System_shutdown`** | `1.16 s` (✅ **1.00x**) | `725.84 ms` (✅ **1.60x faster**) | `396.10 ms` (🚀 **2.93x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

