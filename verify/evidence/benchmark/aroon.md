# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.189 | 5.29M | 0.193 | 5.18M | 0.042 | 0.22× | 0.22× |
| 10,000 | 1.654 | 6.05M | 1.656 | 6.04M | 0.162 | 0.10× | 0.10× |
| 100,000 | 16.575 | 6.03M | 16.830 | 5.94M | 1.113 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.117 | 1.04× |
| 1 | 5 | 0.423 | 0.484 | 1.14× |
| 1 | 10 | 0.648 | 0.948 | 1.46× |
| 10 | 1 | 0.076 | 0.096 | 1.27× |
| 10 | 5 | 0.310 | 0.463 | 1.49× |
| 10 | 10 | 0.647 | 0.974 | 1.50× |
| 100 | 1 | 0.083 | 0.097 | 1.16× |
| 100 | 5 | 0.334 | 0.485 | 1.45× |
| 100 | 10 | 0.642 | 0.944 | 1.47× |
| 1,000 | 1 | 0.248 | 0.105 | 0.42× |
| 1,000 | 5 | 0.410 | 0.514 | 1.25× |
| 1,000 | 10 | 0.734 | 1.093 | 1.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
