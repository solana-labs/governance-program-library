# governance-program-library

### Test

Unit tests contained within all projects can be run with:
```bash
$ cargo test      # <-- runs host-based tests
$ cargo test-bpf  # <-- runs BPF program tests
```

To run a specific program's tests, such as for NFT Voter Plugin:
```bash
$ cd programs/nft-voter
$ cargo test      # <-- runs host-based tests
$ cargo test-bpf  # <-- runs BPF program tests
```

To run a specific test, give the test name (doesnt include the file name)
```bash
$ cargo test test_create_governance_token_holding_account -- --exact    # <-- runs host-based tests
$ cargo test-bpf test_create_governance_token_holding_account -- --exact  # <-- runs BPF program tests
```