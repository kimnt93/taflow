# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.35M | 0.041 | 24.49M | 0.033 | 0.63× | 0.80× |
| 10,000 | 0.325 | 30.80M | 0.308 | 32.43M | 0.081 | 0.25× | 0.26× |
| 100,000 | 2.934 | 34.08M | 2.951 | 33.89M | 0.471 | 0.16× | 0.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.105 | 0.84× |
| 1 | 5 | 0.461 | 0.492 | 1.07× |
| 1 | 10 | 0.654 | 0.910 | 1.39× |
| 10 | 1 | 0.072 | 0.088 | 1.22× |
| 10 | 5 | 0.320 | 0.419 | 1.31× |
| 10 | 10 | 0.622 | 0.912 | 1.47× |
| 100 | 1 | 0.072 | 0.087 | 1.21× |
| 100 | 5 | 0.318 | 0.427 | 1.34× |
| 100 | 10 | 0.644 | 0.955 | 1.48× |
| 1,000 | 1 | 0.107 | 0.094 | 0.88× |
| 1,000 | 5 | 0.309 | 0.483 | 1.56× |
| 1,000 | 10 | 0.676 | 0.956 | 1.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
