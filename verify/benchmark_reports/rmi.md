# RelativeMomentumIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.90M | 0.009 | 112.52M | 0.031 | 3.02× | 3.44× |
| 10,000 | 0.074 | 135.92M | 0.071 | 140.39M | 0.308 | 4.18× | 4.32× |
| 100,000 | 0.750 | 133.36M | 0.704 | 142.01M | 2.871 | 3.83× | 4.08× |
| 1,000,000 | 7.276 | 137.44M | 6.673 | 149.85M | 45.671 | 6.28× | 6.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.081 | 0.85× |
| 1 | 5 | 0.281 | 0.232 | 0.83× |
| 1 | 10 | 0.445 | 0.437 | 0.98× |
| 10 | 1 | 0.049 | 0.047 | 0.96× |
| 10 | 5 | 0.207 | 0.197 | 0.95× |
| 10 | 10 | 0.457 | 0.446 | 0.98× |
| 100 | 1 | 0.049 | 0.052 | 1.06× |
| 100 | 5 | 0.217 | 0.218 | 1.01× |
| 100 | 10 | 0.457 | 0.495 | 1.08× |
| 1,000 | 1 | 0.056 | 0.079 | 1.41× |
| 1,000 | 5 | 0.241 | 0.400 | 1.66× |
| 1,000 | 10 | 0.492 | 0.770 | 1.57× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.241 | 0.164 | 6.08M | 2712.262 | 16501.05× |
| 100,000 | 10 | 0.942 | 0.547 | 18.27M | 2817.195 | 5148.25× |
| 100,000 | 1,000 | 9.320 | 7.969 | 125.48M | 2927.111 | 367.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 107.69M | 127.32M | 1.00× | 3.58M | 4.15M | 1.00× | 30.50M |
| 5 | 141.14M | 158.17M | 1.24× | 2.63M | 2.57M | 0.62× | 29.98M |
| 10 | 263.17M | 305.14M | 2.40× | 2.28M | 2.49M | 0.60× | 30.67M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
