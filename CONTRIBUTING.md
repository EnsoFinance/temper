## Contributing to transaction-simulator

We welcome all contributions! We just ask a few small things.

### Formatting & Clippy

You should run the below and ensure all pass before committing, as they are checked by CI:

```bash
$ cargo fmt --all --check
$ cargo clippy --all --all-features -- -D warnings
```

These help keep the code ✨clean✨.

### Tests

To run tests, simply run:

```bash
$ cargo test
```

 - For any new features you add, there should also be new unit or integration tests added.
 - For any bugs you fix, there should be regression tests added to ensure the bug doesn't crop up again.
