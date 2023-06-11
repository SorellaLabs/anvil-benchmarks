![Race of the forks](/assets/RaceOfTheForks.png)

# Anvil provider benchmarks

This repository benchmarks Anvil providers by running Ethereum block simulations and invoking the systemShutdown method from Convex Finance, known for high gas consumption and multiple token transfers.

## Background

The Ipc and Ethers Reth middleware functionality has been developed by Sorella Labs for our back-testing needs. Our hope is to contribute these developments to foundry in the near future.

## Benchmarks

The repository contains the following benchmarks:

1. **Convex Finance System Shutdown Simulation**: This benchmark simulates the system shutdown of the Convex Finance protocol.

2. **Block Simulation Benchmark**: This benchmark replicates the execution of Ethereum blocks as both a standalone block simulation & sequence block simulation.

## Methodology

The benchmarking procedure uses Criterion.rs, a Rust benchmarking library, and is conducted in the following manner:

1. **Convex Finance System Shutdown Simulation**: Each test begins by warming up the CPU and filling up the cache by repeatedly executing the benchmark function. Then, a system shutdown operation for the Convex Finance protocol is simulated using various Anvil provider configurations. Each operation is timed and the results are logged.

2. **Single Block Simulation**: For each Ethereum block to be simulated, the function is executed in the same way as the system shutdown operation. Each transaction within the block is executed and mined one at a time.

3. **Sequential Block Simulation**: The same procedure as the single block simulation is repeated, but for a sequence of Ethereum blocks. Here, each block and its transactions are executed and mined in sequence.

For all stages, the logged times are statistically analyzed using Criterion.rs. Results are categorized using a version of Tukey's Method and a linear regression generates a confidence interval. Performance changes are identified by comparing current results to previous ones.

The benchmark tests are conducted on an AWS is4gen.2xlarge instance at a specific Ethereum mainnet block with various providers. Storage caching is disabled and cache is cleared after each run for accurate performance measurements.

## Results

## Setup

You will need Rust and Cargo installed to run these benchmarks. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

Next, clone the repository and navigate to the root directory:

```
git clone https://github.com/AnvilEthereum/anvil-benchmarks.git
cd anvil-benchmarks
```

Note: Running these benchmarks requires a local Reth archive node.

## Running Benchmarks

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

To run the Convex Finance System Shutdown benchmark:

```
cargo bench --bench anvil_sys_shutdown_benchmarks
```

To run the Block Simulation benchmark:

```
cargo bench --bench anvil_block_mine_benchmarks
```

## Reporting

Benchmark results are displayed in the console. For more detailed statistics and insights, Criterion saves data in the target/criterion directory. Open the HTML report file for each benchmark in a web browser:

- For the Convex Finance System Shutdown benchmark, open `target/criterion/Convex System Shutdown Simulation/report/index.html`.
- For the Block Simulation benchmark, open `target/criterion/Block Simulation/report/index.html`.

## Contributions

Contributions to this project are welcome! If you have any suggestions or find any issues, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
