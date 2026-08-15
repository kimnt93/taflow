# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 133.19M | 0.006 | 162.55M | 0.037 | 4.90× | 5.98× |
| 10,000 | 0.058 | 172.59M | 0.059 | 170.10M | 0.089 | 1.54× | 1.52× |
| 100,000 | 0.533 | 187.64M | 0.506 | 197.56M | 0.571 | 1.07× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.143 | 1.28× |
| 1 | 5 | 0.217 | 0.492 | 2.27× |
| 1 | 10 | 0.385 | 0.933 | 2.42× |
| 10 | 1 | 0.047 | 0.094 | 2.00× |
| 10 | 5 | 0.180 | 0.507 | 2.81× |
| 10 | 10 | 0.464 | 0.966 | 2.08× |
| 100 | 1 | 0.045 | 0.099 | 2.21× |
| 100 | 5 | 0.180 | 0.438 | 2.43× |
| 100 | 10 | 0.398 | 0.966 | 2.42× |
| 1,000 | 1 | 0.054 | 0.100 | 1.84× |
| 1,000 | 5 | 0.201 | 0.486 | 2.42× |
| 1,000 | 10 | 0.427 | 0.990 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
