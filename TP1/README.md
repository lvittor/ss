# Build and run with visualization

#### Implementation

Run CIM method by default

```
make run-impl
```

Run with bruteforce

```
make run-impl ARGS="-o /dev/stdout -b"
```

#### Visualization

```
make run-viz
```

#### Run everything together

```
make run-impl USE_DOCKER=FALSE ARGS="-o /dev/stdout" | make run-viz USE_DOCKER=FALSE ARGS="-o /dev/stdin"
```
