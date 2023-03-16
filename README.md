```
docker compose build
docker compose up -d
```

go to http://127.0.0.1:8899/lab (password: ss)



# Rust projects

#### Make arguments:

```
USE_DOCKER=TRUE (default) | FALSE
ARGS=
BIN=
PACKAGE=
```


## Show targets

```bash
make -f Makefile.rust
```

## Build all targets

```bash
make -f Makefile.rust build
```

## Build and run

```bash
make -f Makefile.rust run BIN=$(BINARY-NAME) ARGS=
```

## Run

### TP1 Specific

#### Implementation

Run CIM method by default

```
make -f Makefile.rust run-impl
```

Run with bruteforce

```
make -f Makefile.rust run-impl ARGS="-o /dev/stdout -b"
```

#### Visualization

```
make -f Makefile.rust run-viz
```
