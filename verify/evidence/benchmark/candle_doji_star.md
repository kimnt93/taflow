# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.09M | 0.016 | 64.25M | 0.038 | 1.97× | 2.43× |
| 10,000 | 0.153 | 65.28M | 0.151 | 66.35M | 0.134 | 0.88× | 0.89× |
| 100,000 | 1.505 | 66.44M | 1.522 | 65.69M | 1.088 | 0.72× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.144 | 1.20× |
| 1 | 5 | 0.446 | 0.433 | 0.97× |
| 1 | 10 | 0.523 | 0.911 | 1.74× |
| 10 | 1 | 0.053 | 0.086 | 1.62× |
| 10 | 5 | 0.280 | 0.459 | 1.64× |
| 10 | 10 | 0.558 | 0.886 | 1.59× |
| 100 | 1 | 0.054 | 0.086 | 1.58× |
| 100 | 5 | 0.262 | 0.430 | 1.64× |
| 100 | 10 | 0.566 | 0.915 | 1.62× |
| 1,000 | 1 | 0.068 | 0.107 | 1.58× |
| 1,000 | 5 | 0.260 | 0.486 | 1.87× |
| 1,000 | 10 | 0.565 | 1.077 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
