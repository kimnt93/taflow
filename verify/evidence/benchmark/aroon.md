# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 29.86M | 0.032 | 31.33M | 0.046 | 1.38× | 1.44× |
| 10,000 | 0.298 | 33.55M | 0.302 | 33.10M | 0.183 | 0.61× | 0.60× |
| 100,000 | 3.141 | 31.84M | 2.938 | 34.04M | 1.158 | 0.37× | 0.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.115 | 0.84× |
| 1 | 5 | 0.299 | 0.498 | 1.66× |
| 1 | 10 | 0.515 | 1.007 | 1.96× |
| 10 | 1 | 0.059 | 0.097 | 1.64× |
| 10 | 5 | 0.235 | 0.461 | 1.96× |
| 10 | 10 | 0.475 | 1.020 | 2.15× |
| 100 | 1 | 0.061 | 0.106 | 1.73× |
| 100 | 5 | 0.251 | 0.471 | 1.88× |
| 100 | 10 | 0.521 | 0.999 | 1.92× |
| 1,000 | 1 | 0.094 | 0.115 | 1.22× |
| 1,000 | 5 | 0.299 | 0.550 | 1.84× |
| 1,000 | 10 | 0.550 | 1.149 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
