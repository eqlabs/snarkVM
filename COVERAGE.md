## Codecov Coverage

The test coverage currently relies on:
- Circle CI setup for Github and this branch in particular
- Codecov setup for this repository
- `llvm-cov` instrumentation for capturing more precise coverage during test runs
- `nextest` for better parallelized tests
- `tmux` for doing the coverage run in detached session 

Instructions:
1. Use a beefy machine (min 128GB memory recommended, as much cores as possible). The step 3 will take days so if you're using cloud instances, choose and prepare the node accordingly.
2. Install needed plugins:
```
cargo install --locked cargo-llvm-cov
cargo install --locked cargo-nextest
```
3. Start a new tmux session and run the `lcov-nextest-coverage.sh`:
``` 
tmux
./lcov-nextest-coverage.sh
```
4. Compress and add the `lcov.info` coverage files to git
```
find . -name "lcov*.info" | while read file; do tar -czf "${file}.tar.gz" "$file"; done
find . -name "lcov*.tar.gz" -exec git add {} \;
```
5. Push the changed coverage files to branch so that CI will pick them up.

