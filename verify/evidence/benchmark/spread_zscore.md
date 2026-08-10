# SpreadZScore benchmark (`rolling hedged-spread z-score` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.56M | 0.089 | 11.26M | 0.406 | 4.69× | 4.57× |
| 10,000 | 0.865 | 11.56M | 0.886 | 11.29M | 2.686 | 3.10× | 3.03× |
| 100,000 | 8.490 | 11.78M | 9.105 | 10.98M | 34.560 | 4.07× | 3.80× |
| 1,000,000 | 85.405 | 11.71M | 88.745 | 11.27M | 366.246 | 4.29× | 4.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.206 | 2.27× |
| 1 | 5 | 0.324 | 0.786 | 2.42× |
| 1 | 10 | 0.498 | 1.711 | 3.43× |
| 10 | 1 | 0.055 | 0.161 | 2.92× |
| 10 | 5 | 0.246 | 0.778 | 3.16× |
| 10 | 10 | 0.510 | 1.621 | 3.18× |
| 100 | 1 | 0.061 | 0.257 | 4.20× |
| 100 | 5 | 0.231 | 1.434 | 6.20× |
| 100 | 10 | 0.507 | 2.825 | 5.57× |
| 1,000 | 1 | 0.144 | 0.507 | 3.53× |
| 1,000 | 5 | 0.287 | 1.808 | 6.29× |
| 1,000 | 10 | 0.577 | 4.001 | 6.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
