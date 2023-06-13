![Race of the forks](/assets/RaceOfTheForks.png)

# Anvil provider benchmarks

This repository benchmarks Anvil providers by running Ethereum block simulations and invoking the systemShutdown method from Convex Finance, known for high gas consumption and multiple token transfers.

## Background

The Ipc and Ethers Reth middleware functionality has been developed by Sorella Labs for our back-testing needs. Our hope is to contribute these developments to Foundry in the near future.

## Benchmarks

The repository contains the following benchmarks:

1. **Convex Finance System Shutdown Simulation**: This benchmark simulates the system shutdown of the Convex Finance protocol.

2. **Block Simulation Benchmark**: This benchmark replicates the execution of high-gas Ethereum blocks as both a standalone block simulation & sequence block simulation.

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
os: Amazon Linux AMI 2023.0.0 [64-bit]

```

## Results

The benchmarks were all performed over 1000 iterations. The results are summarized below.

### Sequential Simulation

|                                       | `Local_Http`            | `Ipc`                          | `ethers-reth`                  |
| :------------------------------------ | :---------------------- | :----------------------------- | :----------------------------- |
| **`Blocks 14,556,786 -> 14,556,795`** | `4.10 s` (✅ **1.00x**) | `3.39 s` (✅ **1.21x faster**) | `1.73 s` (🚀 **2.37x faster**) |

### Individual Block Simulation

|                                      | `Local_Http`               | `Ipc`                             | `ethers-reth`                     |
| :----------------------------------- | :------------------------- | :-------------------------------- | :-------------------------------- |
| **`Block: 0, TotalGas: 30,312,275`** | `802.21 ms` (✅ **1.00x**) | `670.99 ms` (✅ **1.20x faster**) | `342.05 ms` (🚀 **2.35x faster**) |
| **`Block: 1, TotalGas: 26,490,097`** | `728.31 ms` (✅ **1.00x**) | `617.74 ms` (✅ **1.18x faster**) | `310.55 ms` (🚀 **2.35x faster**) |
| **`Block: 2, TotalGas: 1,973,605`**  | `105.19 ms` (✅ **1.00x**) | `93.00 ms` (✅ **1.13x faster**)  | `63.51 ms` (✅ **1.66x faster**)  |
| **`Block: 3, TotalGas: 30,209,666`** | `920.93 ms` (✅ **1.00x**) | `777.15 ms` (✅ **1.19x faster**) | `380.73 ms` (🚀 **2.42x faster**) |
| **`Block: 4, TotalGas: 30,248,521`** | `883.31 ms` (✅ **1.00x**) | `735.84 ms` (✅ **1.20x faster**) | `372.69 ms` (🚀 **2.37x faster**) |
| **`Block: 5, TotalGas: 22,259,893`** | `700.30 ms` (✅ **1.00x**) | `581.94 ms` (✅ **1.20x faster**) | `293.28 ms` (🚀 **2.39x faster**) |
| **`Block: 6, TotalGas: 6,195,368`**  | `219.66 ms` (✅ **1.00x**) | `189.43 ms` (✅ **1.16x faster**) | `109.54 ms` (🚀 **2.01x faster**) |
| **`Block: 7, TotalGas: 4,067,167`**  | `180.92 ms` (✅ **1.00x**) | `155.24 ms` (✅ **1.17x faster**) | `88.21 ms` (🚀 **2.05x faster**)  |
| **`Block: 8, TotalGas: 18,144,161`** | `565.45 ms` (✅ **1.00x**) | `478.88 ms` (✅ **1.18x faster**) | `246.88 ms` (🚀 **2.29x faster**) |
| **`Block: 9, TotalGas: 26,759,449`** | `718.40 ms` (✅ **1.00x**) | `594.82 ms` (✅ **1.21x faster**) | `299.91 ms` (🚀 **2.40x faster**) |

### Convex Finance System Shutdown

|                       | `Local Http`               | `Ipc`                             | `ethers-reth`                     |
| :-------------------- | :------------------------- | :-------------------------------- | :-------------------------------- |
| **`System_shutdown`** | `804.77 ms` (✅ **1.00x**) | `615.53 ms` (✅ **1.31x faster**) | `291.17 ms` (🚀 **2.76x faster**) |

---

### Charts

This chart shows the relationship between function and iteration time. The thickness of the shaded region indicates the probability that a measurement of the given function would take a particular length of time.

#### Sequential Simulation

![System Shutdown Violin Chart](target/criterion/reports/Sequential%20Simulation/violin.svg)

- As we can see here, ethers-reth is not only faster, but it is also more consistent.

#### Convex Finance System Shutdown

![System Shutdown Violin Chart](target/criterion/reports/Convex%20Finance%20System%20Shutdown/violin.svg)

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

Prior to initiating the benchmarks, it's advised to reduce the sample size. With the current setting, each benchmark is executed 1000 times, which can significantly prolong the completion time.
To run all benchmarks & generate a report in the BENCHMARSK.md file:

```
cargo criterion --message-format=json | criterion-table > BENCHMARKS.md
```

### Reporting

Benchmark results are displayed in the console. For more detailed statistics, Criterion saves the data in target/criterion. Open the `index.html` file generated by Criterion in a web browser for more insights & graphs.

relative path: `target/criterion/reports/index.html`

### Contributions

Contributions to this project are welcome! If you have any suggestions or find any issues, feel free to open an issue or submit a pull request.

### License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
