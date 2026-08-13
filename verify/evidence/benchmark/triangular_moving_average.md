# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.27M | 0.039 | 25.81M | 0.034 | 0.79× | 0.88× |
| 10,000 | 0.304 | 32.89M | 0.314 | 31.84M | 0.062 | 0.20× | 0.20× |
| 100,000 | 2.933 | 34.09M | 2.851 | 35.07M | 0.314 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.128 | 1.11× |
| 1 | 5 | 0.364 | 0.474 | 1.30× |
| 1 | 10 | 0.615 | 0.934 | 1.52× |
| 10 | 1 | 0.069 | 0.095 | 1.38× |
| 10 | 5 | 0.299 | 0.456 | 1.52× |
| 10 | 10 | 0.609 | 0.951 | 1.56× |
| 100 | 1 | 0.066 | 0.095 | 1.45× |
| 100 | 5 | 0.301 | 0.466 | 1.55× |
| 100 | 10 | 0.647 | 0.947 | 1.46× |
| 1,000 | 1 | 0.096 | 0.098 | 1.02× |
| 1,000 | 5 | 0.317 | 0.483 | 1.53× |
| 1,000 | 10 | 0.675 | 0.995 | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
