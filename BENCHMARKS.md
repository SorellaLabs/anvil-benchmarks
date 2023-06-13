# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Block Simulation](#block-simulation)
    - [Anvil](#anvil)

## Benchmark Results

### Block Simulation

|                              | `Local_Http`              | `Ipc`                            | `ethers_reth_middleware`          | `All_blocks`            |
|:-----------------------------|:--------------------------|:---------------------------------|:----------------------------------|:----------------------- |
| **`Local_Http`**             | `N/A`                     | `N/A`                            | `N/A`                             | `5.34 s` (✅ **1.00x**)  |
| **`Block_0`**                | `1.08 s` (✅ **1.00x**)    | `789.75 ms` (✅ **1.37x faster**) | `408.03 ms` (🚀 **2.64x faster**)  | `N/A`                   |
| **`Block_1`**                | `942.26 ms` (✅ **1.00x**) | `724.89 ms` (✅ **1.30x faster**) | `350.74 ms` (🚀 **2.69x faster**)  | `N/A`                   |
| **`Block_2`**                | `163.18 ms` (✅ **1.00x**) | `123.01 ms` (✅ **1.33x faster**) | `59.83 ms` (🚀 **2.73x faster**)   | `N/A`                   |
| **`Block_3`**                | `1.29 s` (✅ **1.00x**)    | `865.22 ms` (✅ **1.49x faster**) | `493.79 ms` (🚀 **2.61x faster**)  | `N/A`                   |
| **`Block_4`**                | `1.24 s` (✅ **1.00x**)    | `857.00 ms` (✅ **1.45x faster**) | `443.70 ms` (🚀 **2.79x faster**)  | `N/A`                   |
| **`Block_5`**                | `866.88 ms` (✅ **1.00x**) | `765.65 ms` (✅ **1.13x faster**) | `427.86 ms` (🚀 **2.03x faster**)  | `N/A`                   |
| **`Block_6`**                | `375.48 ms` (✅ **1.00x**) | `241.81 ms` (✅ **1.55x faster**) | `117.91 ms` (🚀 **3.18x faster**)  | `N/A`                   |
| **`Block_7`**                | `257.22 ms` (✅ **1.00x**) | `202.51 ms` (✅ **1.27x faster**) | `121.65 ms` (🚀 **2.11x faster**)  | `N/A`                   |
| **`Block_8`**                | `770.41 ms` (✅ **1.00x**) | `604.47 ms` (✅ **1.27x faster**) | `310.30 ms` (🚀 **2.48x faster**)  | `N/A`                   |
| **`Block_9`**                | `873.47 ms` (✅ **1.00x**) | `745.34 ms` (✅ **1.17x faster**) | `373.83 ms` (🚀 **2.34x faster**)  | `N/A`                   |
| **`Ipc`**                    | `N/A`                     | `N/A`                            | `N/A`                             | `3.71 s` (✅ **1.00x**)  |
| **`ethers_reth_middleware`** | `N/A`                     | `N/A`                            | `N/A`                             | `1.99 s` (✅ **1.00x**)  |

### Anvil

|                       | `Local Http`           | `Ipc`                            | `ethers-reth`                     |
|:----------------------|:-----------------------|:---------------------------------|:--------------------------------- |
| **`System_shutdown`** | `1.07 s` (✅ **1.00x**) | `767.80 ms` (✅ **1.40x faster**) | `380.97 ms` (🚀 **2.82x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

