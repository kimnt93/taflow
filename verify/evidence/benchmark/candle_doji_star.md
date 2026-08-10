# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.21M | 0.018 | 56.34M | 0.034 | 1.85× | 1.92× |
| 10,000 | 0.153 | 65.49M | 0.176 | 56.97M | 0.133 | 0.87× | 0.76× |
| 100,000 | 1.488 | 67.20M | 1.746 | 57.29M | 1.042 | 0.70× | 0.60× |
| 1,000,000 | 15.408 | 64.90M | 17.686 | 56.54M | 10.631 | 0.69× | 0.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.135 | 1.46× |
| 1 | 5 | 0.319 | 0.462 | 1.45× |
| 1 | 10 | 0.574 | 0.959 | 1.67× |
| 10 | 1 | 0.058 | 0.084 | 1.45× |
| 10 | 5 | 0.261 | 0.429 | 1.64× |
| 10 | 10 | 0.547 | 0.959 | 1.75× |
| 100 | 1 | 0.057 | 0.097 | 1.70× |
| 100 | 5 | 0.274 | 0.445 | 1.62× |
| 100 | 10 | 0.568 | 0.978 | 1.72× |
| 1,000 | 1 | 0.077 | 0.094 | 1.22× |
| 1,000 | 5 | 0.278 | 0.514 | 1.85× |
| 1,000 | 10 | 0.594 | 1.031 | 1.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
