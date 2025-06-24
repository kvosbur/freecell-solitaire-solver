# FreeCell Solver

This application finds solutions to FreeCell solitaire games using the shared game-engine library.

## Building

From the workspace root, build the solver:

```sh
cargo build --package freecell-solver --release
```

## Running

You can run the solver with various strategies and modes using the CLI:

### List Available Strategies

```sh
cargo run --package freecell-solver --bin solver -- --list-strategies
```

### Run with a Specific Strategy

```sh
cargo run --package freecell-solver --bin solver -- --strategy strat1
cargo run --package freecell-solver --bin solver -- --strategy strat5
```

### Run Benchmark Mode

Benchmark mode tests how many moves can be undone and still be solved:

```sh
cargo run --package freecell-solver --bin solver -- --strategy strat5 --benchmark
```

### Run Adhoc Mode (default)

Adhoc mode tests a single scenario (currently 40 moves undone):

```sh
cargo run --package freecell-solver --bin solver -- --strategy strat2
```

### Set a Timeout

You can set a timeout (in seconds) for the solver:

```sh
cargo run --package freecell-solver --bin solver -- --strategy strat5 --timeout 600
```

## Command Line Options

- `--strategy <NAME>`: Select which strategy to use (`strat1`, `strat2`, `strat3`, `strat4`, `strat5`). Default: `strat5`
- `--list-strategies`: List all available strategies and their descriptions
- `--timeout <SECONDS>`: Set the timeout for solving (default: 86400, i.e., 24 hours)
- `--benchmark`: Run benchmark mode (iteratively increases difficulty)
- (No flag): Runs adhoc mode (single test)

## Example Output

```
cargo run --package freecell-solver --bin solver -- --strategy strat5 --benchmark
Using strategy: strat5
Trying to undue 32 moves from solution of length 52
Succeeded with 32 moves undone
...
Benchmark completed. Last successful move count to undue: 37
```

## Example Config File (Template)

*Note: The current CLI does not use a config file, but this is a template for future extension.*

```toml
[solver]
default_strategy = "strat5"
timeout_seconds = 3600

[strategies.strat1]
max_depth = 200

[strategies.strat5]
max_depth = 200
cache_size = 250_000_000
```

## Profiling and Performance

To profile the solver, you can use tools like Valgrind's callgrind:

1. Build the release binary:
   ```
   cargo build --bin solver --release
   ```
2. Run with callgrind:
   ```
   valgrind --tool=callgrind ./target/release/solver
   ```
3. Analyze with:
   ```
   callgrind_annotate callgrind.out.<pid>
   ```

## Development Notes

- Strategies are implemented as plugins and can be selected at runtime.
- See `src/strategies/` for available strategies and their implementations.
- Extend the CLI or config system as needed for more advanced usage.
