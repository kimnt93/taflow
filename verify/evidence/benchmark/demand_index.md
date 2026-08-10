# DemandIndex benchmark (`DemandIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.08M | 0.018 | 55.17M | 0.514 | 38.11× | 28.38× |
| 10,000 | 0.091 | 110.37M | 0.075 | 132.94M | 1.412 | 15.58× | 18.76× |
| 100,000 | 0.543 | 184.16M | 0.517 | 193.48M | 12.651 | 23.30× | 24.48× |
| 1,000,000 | 6.067 | 164.84M | 5.315 | 188.15M | 136.239 | 22.46× | 25.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.485 | 5.59× |
| 1 | 5 | 0.299 | 1.098 | 3.67× |
| 1 | 10 | 0.539 | 2.309 | 4.28× |
| 10 | 1 | 0.061 | 0.216 | 3.53× |
| 10 | 5 | 0.253 | 1.286 | 5.08× |
| 10 | 10 | 0.518 | 2.376 | 4.58× |
| 100 | 1 | 0.058 | 0.231 | 4.00× |
| 100 | 5 | 0.258 | 1.323 | 5.12× |
| 100 | 10 | 0.535 | 2.484 | 4.64× |
| 1,000 | 1 | 0.064 | 0.361 | 5.59× |
| 1,000 | 5 | 0.271 | 1.909 | 7.05× |
| 1,000 | 10 | 0.563 | 3.644 | 6.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
