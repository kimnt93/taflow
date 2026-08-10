# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.86M | 0.020 | 50.91M | 0.044 | 1.53× | 2.23× |
| 10,000 | 0.177 | 56.42M | 0.177 | 56.61M | 0.155 | 0.87× | 0.88× |
| 100,000 | 1.810 | 55.26M | 1.707 | 58.57M | 1.093 | 0.60× | 0.64× |
| 1,000,000 | 16.994 | 58.84M | 16.498 | 60.61M | 10.679 | 0.63× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.134 | 1.55× |
| 1 | 5 | 0.367 | 0.568 | 1.55× |
| 1 | 10 | 0.701 | 1.189 | 1.69× |
| 10 | 1 | 0.069 | 0.106 | 1.55× |
| 10 | 5 | 0.361 | 0.629 | 1.74× |
| 10 | 10 | 0.656 | 1.215 | 1.85× |
| 100 | 1 | 0.074 | 0.112 | 1.51× |
| 100 | 5 | 0.367 | 0.717 | 1.95× |
| 100 | 10 | 0.705 | 1.304 | 1.85× |
| 1,000 | 1 | 0.080 | 0.132 | 1.65× |
| 1,000 | 5 | 0.415 | 0.575 | 1.39× |
| 1,000 | 10 | 0.753 | 1.509 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
