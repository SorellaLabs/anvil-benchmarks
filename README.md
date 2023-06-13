![Race of the forks](/assets/RaceOfTheForks.png)

# Anvil provider benchmarks

This repository benchmarks Anvil providers by running Ethereum block simulations and invoking the systemShutdown method from Convex Finance, known for high gas consumption and multiple token transfers.

## Background

The Ipc and Ethers Reth middleware functionality has been developed by Sorella Labs for our back-testing needs. Our hope is to contribute these developments to foundry in the near future.

## Benchmarks

The repository contains the following benchmarks:

1. **Convex Finance System Shutdown Simulation**: This benchmark simulates the system shutdown of the Convex Finance protocol.

2. **Block Simulation Benchmark**: This benchmark replicates the execution of high gas Ethereum blocks as both a standalone block simulation & sequence block simulation.

## Methodology

Using the Criterion.rs library for Rust benchmarking, we follow these procedures:

1. **Convex Finance System Shutdown Simulation**: Each test begins by warming up the CPU and filling up the cache by repeatedly executing the benchmark function. Then, a system shutdown operation for the Convex Finance protocol is simulated using various Anvil provider configurations. Each operation is timed and the results are logged.

2. **Single Block Simulation**: This simulates a single Ethereum block repeatedly, with each instance requiring a state fork by Anvil. The block is executed and mined, and the time taken to execute the block is logged.

3. **Sequential Block Simulation**: This process extends to a sequence of Ethereum blocks. Anvil forks the state for the first block only and then executes transactions continuously across the sequence of blocks.

At each stage, logged times are statistically analyzed with Criterion.rs.

### Environment

Benchmark tests are conducted on an AWS is4gen.2xlarge instance. To ensure accurate performance measurements, storage caching is disabled.
The environment setup is as follows:

```
# cargo +nightly version -v
cargo 1.72.0-nightly (49b6d9e17 2023-06-09)
release: 1.72.0-nightly
commit-hash: 49b6d9e179a91cf7645142541c9563443f64bf2b
commit-date: 2023-06-09
host: aarch64-unknown-linux-gnu
libgit2: 1.6.4 (sys:0.17.1 vendored)
libcurl: 8.1.2-DEV (sys:0.4.63+curl-8.1.2 vendored ssl:OpenSSL/1.1.1t)
ssl: OpenSSL 1.1.1t  7 Feb 2023
os: Amazon Linux AMI 2023.0.0 [64-bit]

```

## Results

The benchmarks were all performed over 1000 iterations. The results are summarized below.

### Sequential Simulation

|                                       | `Local_Http`            | `Ipc`                          | `ethers_reth_middleware`       |
| :------------------------------------ | :---------------------- | :----------------------------- | :----------------------------- |
| **`Blocks 14,556,786 -> 14,556,795`** | `4.22 s` (✅ **1.00x**) | `3.31 s` (✅ **1.27x faster**) | `1.72 s` (🚀 **2.45x faster**) |

### Individual Block Simulation

|                                      | `Local_Http`               | `Ipc`                             | `ethers_reth_middleware`          |
| :----------------------------------- | :------------------------- | :-------------------------------- | :-------------------------------- |
| **`Block: 0, TotalGas: 30,312,275`** | `795.34 ms` (✅ **1.00x**) | `686.04 ms` (✅ **1.16x faster**) | `335.74 ms` (🚀 **2.37x faster**) |
| **`Block: 1, TotalGas: 26,490,097`** | `745.08 ms` (✅ **1.00x**) | `624.51 ms` (✅ **1.19x faster**) | `326.61 ms` (🚀 **2.28x faster**) |
| **`Block: 2, TotalGas: 1,973,605`**  | `101.47 ms` (✅ **1.00x**) | `89.23 ms` (✅ **1.14x faster**)  | `64.81 ms` (✅ **1.57x faster**)  |
| **`Block: 3, TotalGas: 30,209,666`** | `884.96 ms` (✅ **1.00x**) | `744.28 ms` (✅ **1.19x faster**) | `369.63 ms` (🚀 **2.39x faster**) |
| **`Block: 4, TotalGas: 30,248,521`** | `909.50 ms` (✅ **1.00x**) | `748.81 ms` (✅ **1.21x faster**) | `366.58 ms` (🚀 **2.48x faster**) |
| **`Block: 5, TotalGas: 22,259,893`** | `689.22 ms` (✅ **1.00x**) | `591.46 ms` (✅ **1.17x faster**) | `307.42 ms` (🚀 **2.24x faster**) |
| **`Block: 6, TotalGas: 6,195,368`**  | `207.07 ms` (✅ **1.00x**) | `187.08 ms` (✅ **1.11x faster**) | `111.68 ms` (🚀 **1.85x faster**) |
| **`Block: 7, TotalGas: 4,067,167`**  | `184.81 ms` (✅ **1.00x**) | `152.36 ms` (✅ **1.21x faster**) | `83.47 ms` (🚀 **2.21x faster**)  |
| **`Block: 8, TotalGas: 18,144,161`** | `577.01 ms` (✅ **1.00x**) | `483.47 ms` (✅ **1.19x faster**) | `238.91 ms` (🚀 **2.42x faster**) |
| **`Block: 9, TotalGas: 26,759,449`** | `758.92 ms` (✅ **1.00x**) | `582.24 ms` (✅ **1.30x faster**) | `305.53 ms` (🚀 **2.48x faster**) |

### Convex Finance System Shutdown

|                       | `Local Http`               | `Ipc`                             | `ethers-reth`                     |
| :-------------------- | :------------------------- | :-------------------------------- | :-------------------------------- |
| **`System_shutdown`** | `876.58 ms` (✅ **1.00x**) | `587.53 ms` (✅ **1.49x faster**) | `276.74 ms` (🚀 **3.17x faster**) |

### Charts

This chart shows the relationship between function/parameter and iteration time. The thickness of the shaded region indicates the probability that a measurement of the given function/parameter would take a particular length of time.

#### Sequential Simulation

![System Shutdown Violin Chart](/target/criterion/reports/Sequential Simulation/violin.svg)

- As we can see here, ethers-reth is not only faster, it is also more consistent.

#### Convex Finance System Shutdown

![System Shutdown Violin Chart](/target/criterion/reports/Convex Finance System Shutdown/violin.svg)

### Setup

You will need Rust and Cargo installed to run these benchmarks. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

Next, clone the repository and navigate to the root directory:

```
git clone https://github.com/AnvilEthereum/anvil-benchmarks.git
cd anvil-benchmarks
```

Note: Running these benchmarks requires a local Reth archive node.

### Running Benchmarks

Once you have the necessary prerequisites installed and cloned the repository, you can run the benchmarks.

1. Copy the template for the environment variables configuration:

```
cp .env.example .env
```

In the resulting `.env` file, fill out the following fields:

- `ETH_RPC_URL`: This should be a URL to an Ethereum archive node.
- `ETH_RPC_URL_LOCAL`: This should be a URL to your local Ethereum node.
- `ETH_IPC_PATH`: IPC path to the local node.
- `ETH_DB_PATH`: Reth DB path to the local archive node.

2. Load the environment variables into your shell:

```
source .env
```

3. Run the benchmarks:

To run all benchmarks & generate a report in the BENCHMARSK.md file:

```
cargo criterion --message-format=json | criterion-table > BENCHMARKS.md
```

### Reporting

Benchmark results are displayed in the console. For more detailed statistics, Criterion saves the data in target/criterion. Open the `index.html` file generated by Criterion in a web browser for more insights & graphs.

### Contributions

Contributions to this project are welcome! If you have any suggestions or find any issues, feel free to open an issue or submit a pull request.

### License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
