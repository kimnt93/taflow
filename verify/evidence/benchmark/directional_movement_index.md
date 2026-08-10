# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.48M | 0.014 | 71.77M | 0.039 | 2.26× | 2.77× |
| 10,000 | 0.103 | 97.45M | 0.103 | 97.41M | 0.114 | 1.11× | 1.11× |
| 100,000 | 0.969 | 103.22M | 0.929 | 107.64M | 0.826 | 0.85× | 0.89× |
| 1,000,000 | 10.051 | 99.49M | 11.464 | 87.23M | 9.118 | 0.91× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.120 | 1.36× |
| 1 | 5 | 0.313 | 0.482 | 1.54× |
| 1 | 10 | 0.519 | 0.939 | 1.81× |
| 10 | 1 | 0.051 | 0.089 | 1.74× |
| 10 | 5 | 0.243 | 0.428 | 1.76× |
| 10 | 10 | 0.471 | 0.910 | 1.93× |
| 100 | 1 | 0.054 | 0.090 | 1.66× |
| 100 | 5 | 0.234 | 0.434 | 1.85× |
| 100 | 10 | 0.856 | 0.970 | 1.13× |
| 1,000 | 1 | 0.071 | 0.101 | 1.42× |
| 1,000 | 5 | 0.307 | 0.537 | 1.75× |
| 1,000 | 10 | 0.589 | 1.091 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
