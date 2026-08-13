# AwesomeOscillator benchmark (`AwesomeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.257 | 3.89M | 0.241 | 4.15M | 0.223 | 0.87× | 0.93× |
| 10,000 | 2.348 | 4.26M | 2.388 | 4.19M | 0.826 | 0.35× | 0.35× |
| 100,000 | 23.984 | 4.17M | 24.697 | 4.05M | 6.656 | 0.28× | 0.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.213 | 0.285 | 1.34× |
| 1 | 5 | 0.412 | 1.405 | 3.41× |
| 1 | 10 | 0.661 | 2.599 | 3.93× |
| 10 | 1 | 0.072 | 0.244 | 3.42× |
| 10 | 5 | 0.315 | 1.364 | 4.32× |
| 10 | 10 | 0.648 | 2.449 | 3.78× |
| 100 | 1 | 0.100 | 0.249 | 2.49× |
| 100 | 5 | 0.323 | 1.372 | 4.25× |
| 100 | 10 | 0.621 | 2.707 | 4.36× |
| 1,000 | 1 | 0.320 | 0.305 | 0.95× |
| 1,000 | 5 | 0.478 | 1.733 | 3.62× |
| 1,000 | 10 | 0.931 | 3.192 | 3.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
