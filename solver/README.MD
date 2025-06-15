# Profiling Code

## Generate binary
Run the following command: `cargo build --bin solver --release`
This will generate an executable at `./target/release/solver`

## Run Valgrind
The tool I noticed to be most helpful when considering cpu usage was callgrind

### Callgrind
I setup the build binary to run in adhoc mode for a couple of minutes timeout on a move that it can't accomplish in that time.
Then call the following command once the binary is built: `valgrind --tool=callgrind ./target/release/solver`
This will generate a file at root level named `callgrind.out.<pid>`

You can then look at the results using the `callgrind_annotate` annotate tool. 
Base: `callgrind_annotate callgrind.out.<pid>`
The command I found most useful was the following but you should mess around with its few options to see what helps you understand better: `callgrind_annotate callgrind.out.<pid> --show-percs=yes --tree=caller --inclusive=yes`

