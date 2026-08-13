# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.65M | 0.043 | 23.21M | 0.036 | 0.82× | 0.84× |
| 10,000 | 0.349 | 28.64M | 0.325 | 30.81M | 0.049 | 0.14× | 0.15× |
| 100,000 | 3.014 | 33.18M | 3.113 | 32.12M | 0.211 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.144 | 1.42× |
| 1 | 5 | 0.435 | 0.532 | 1.22× |
| 1 | 10 | 0.596 | 0.942 | 1.58× |
| 10 | 1 | 0.064 | 0.093 | 1.46× |
| 10 | 5 | 0.307 | 0.456 | 1.48× |
| 10 | 10 | 0.585 | 0.914 | 1.56× |
| 100 | 1 | 0.068 | 0.091 | 1.34× |
| 100 | 5 | 0.295 | 0.425 | 1.44× |
| 100 | 10 | 0.635 | 0.941 | 1.48× |
| 1,000 | 1 | 0.096 | 0.095 | 0.99× |
| 1,000 | 5 | 0.297 | 0.461 | 1.55× |
| 1,000 | 10 | 0.627 | 0.962 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
