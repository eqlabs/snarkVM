#!/bin/bash
RUST_MIN_STACK=67108864
CARGO_LLVM_COV_SETUP=yes

cargo llvm-cov clean --workspace
cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

# The targets are copied from the non-coverage test targets in .circleci/config.yml
pushd algorithms && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/account && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info
popd && pushd circuit/account && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov1.info --no-default-features

popd && pushd circuit/algorithms && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/collections && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info
popd && pushd circuit/collections && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov1.info --no-default-features

popd && pushd circuit/environment && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/network && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/program && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/types && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/types/address && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/types/boolean && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/types/field && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/types/group && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/types/integers && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info -- --ignored

popd && pushd circuit/types/scalar && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd circuit/types/string && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/account && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/algorithms && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/collections && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/network && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/network/environment && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/program && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/types && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/types/address && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/types/boolean && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/types/field && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/types/group && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/types/integers && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/types/scalar && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd console/types/string && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd curves && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd fields && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info
popd && pushd ledger && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov1.info valid_solutions --features=test

popd && pushd ledger/authority && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/block && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/committee && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/narwhal && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/narwhal/batch-certificate && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/narwhal/data && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/narwhal/subdag && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/narwhal/transmission && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/narwhal/transmission-id && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/puzzle && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/puzzle/epoch && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/query && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd ledger/store && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info --features=rocks

popd && pushd ledger/test-helpers && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd parameters && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd synthesizer && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info --lib --bins
popd && pushd synthesizer && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov1.info --test '*'

popd && pushd synthesizer/process && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info
popd && pushd synthesizer/process && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov1.info --features=rocks

popd && pushd synthesizer/program && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info --lib --bins
popd && pushd synthesizer/program && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov1.info -E 'all() - test(/keccak|psd|sha/)'
popd && pushd synthesizer/program && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov2.info keccak --test '*'
popd && pushd synthesizer/program && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov3.info psd --test '*'
popd && pushd synthesizer/program && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov4.info sha --test '*'

popd && pushd synthesizer/snark && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd utilities && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd && pushd utilities/derives && cargo llvm-cov nextest --no-clean --lcov --ignore-run-fail --output-path lcov.info

popd