# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.02M | 0.005 | 218.04M | 0.041 | 5.30× | 9.02× |
| 10,000 | 0.099 | 101.34M | 0.092 | 108.80M | 0.169 | 1.72× | 1.84× |
| 100,000 | 1.157 | 86.47M | 1.162 | 86.07M | 1.472 | 1.27× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.121 | 1.03× |
| 1 | 5 | 0.306 | 0.445 | 1.45× |
| 1 | 10 | 0.391 | 0.897 | 2.29× |
| 10 | 1 | 0.047 | 0.099 | 2.08× |
| 10 | 5 | 0.209 | 0.463 | 2.22× |
| 10 | 10 | 0.380 | 0.923 | 2.43× |
| 100 | 1 | 0.042 | 0.097 | 2.31× |
| 100 | 5 | 0.219 | 0.465 | 2.12× |
| 100 | 10 | 0.460 | 0.945 | 2.05× |
| 1,000 | 1 | 0.057 | 0.104 | 1.82× |
| 1,000 | 5 | 0.187 | 0.499 | 2.66× |
| 1,000 | 10 | 0.412 | 1.097 | 2.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
