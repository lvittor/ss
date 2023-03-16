```
docker compose build
docker compose up -d
```

go to http://127.0.0.1:8899/lab (password: ss)



# Rust projects

Show targets

```bash
make -f Makefile.rust
```

# Build

```bash
make -f Makefile.rust build
```

# Run

## Implementation

Run CIM method by default

```
make -f Makefile.rust run-impl
```

Run with bruteforce

```
make -f Makefile.rust run-impl ARGS="-o /dev/stdout -b"
```

## Visualization

```
make -f Makefile.rust run-viz
```
