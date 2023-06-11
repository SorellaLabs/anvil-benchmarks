![Convex system shutdown simulation](/target/criterion/Convex system shutdown simulation using anvil/report/violin.svg)


# Anvil systemShutdown provider benchmarks

This repository benchmarks the performance of various anvil providers by simulating a call to Convex's systemShutdown method (a method that typically requires a substantial amount of gas and performs numerous token transfers).

## Background

The IPC and Ethers Reth middleware functionality has been developed by Sorella Labs for our back-testing needs. Our hope is to contribute these developments to foundry in the near future.

## Methodology

Benchmarks were run at mainnet block 14,445,961. The benchmarks were conducted using various providers such as Local HTTP, QuickNode HTTP, Infura HTTP, Reth-Ipc, and Ethers-Reth.

1. **No Storage Caching:** Storage caching was disabled to accurately measure the pure performance of the different providers.
2. **Cache Flush:** After each run, the cache was flushed to ensure that subsequent runs start with a clean slate.
3. **Performance Metrics:** For each provider, the Mean Duration, Standard Deviation of Duration, Minimum Duration, and Maximum Duration were measured and recorded.
4. **Environment:** All benchmarks were run on a AWS i3en.3xlarge instance (12 vCPUs and 96 GB of RAM).

## Results

| Anvil Providers | Mean Duration | Std Dev Duration | Min Duration | Max Duration |
| --------------- | ------------- | ---------------- | ------------ | ------------ |
| Local HTTP      | XX.XXXs       | XX.XXXs          | XX.XXXs      | XX.XXXs      |
| QuickNode HTTP  | XX.XXXs       | XX.XXXs          | XX.XXXs      | XX.XXXs      |
| Infura HTTP     | XX.XXXs       | XX.XXXs          | XX.XXXs      | XX.XXXs      |
| Reth-Ipc        | XX.XXXs       | XX.XXXs          | XX.XXXs      | XX.XXXs      |
| Ethers-Reth     | XX.XXXs       | XX.XXXs          | XX.XXXs      | XX.XXXs      |

## Conclusion:

From the obtained results, it can be observed that (insert your conclusion here based on your findings)

It's important to note that performance can vary depending on various factors such as network latency, server load, etc. These benchmarks provide a general idea of the performance capabilities of different Anvil Providers and can be used as a starting point for performance tuning and optimization.

## Usage

1. Copy the template for the environment variables configuration:

```
cp .env.example .env
```

In the resulting `.env` file, fill out the following fields:

- `ETH_RPC_URL`: This should be a URL to an Ethereumta, and archive node. Alchemy provides free archive node da you can set up a node at [Alchemy](https://www.alchemy.com/).
- `ETH_IPC_PATH`: Ipc path to local node
- `ETH_DB_PATH`: Reth db path to local archive node

2. Run source .env to load the environment variables into your shell:

3. Once you have your `.env` setup, you can run the benchmark script with the command:

```
cargo run --release
```

This will output the mean, standard deviation, minimum, and maximum durations for the system shutdown call over a set number of iterations.

## Contributions

Contributions to this project are welcome! If you have any suggestions or find any issues, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

It looks like you're asking to update the README to accommodate for your specific benchmarking script and environment variables setup. Here's a possible revision:
