## rscan
TCP Port Scanner

### How to use rscan:
```
/// scan for single port
cargo r -- -a 127.0.0.1 -s 443

/// scan for range of ports
cargo r -- -a 127.0.0.1 -r 75-85

/// scan for list of ports
cargo r -- -a 127.0.0.1 -l 80,443,1080,445
```