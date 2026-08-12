# RollingSharpe benchmark (`SharpeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.20M | 0.034 | 29.72M | 0.187 | 5.46× | 5.55× |
| 10,000 | 0.286 | 34.94M | 0.289 | 34.65M | 0.542 | 1.89× | 1.88× |
| 100,000 | 2.929 | 34.14M | 3.097 | 32.29M | 4.369 | 1.49× | 1.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.270 | 3.44× |
| 1 | 5 | 0.247 | 1.460 | 5.91× |
| 1 | 10 | 0.520 | 2.747 | 5.28× |
| 10 | 1 | 0.077 | 0.279 | 3.64× |
| 10 | 5 | 0.299 | 1.478 | 4.95× |
| 10 | 10 | 0.568 | 2.908 | 5.12× |
| 100 | 1 | 0.059 | 0.240 | 4.10× |
| 100 | 5 | 0.262 | 1.442 | 5.50× |
| 100 | 10 | 0.639 | 5.605 | 8.76× |
| 1,000 | 1 | 0.139 | 0.393 | 2.82× |
| 1,000 | 5 | 0.442 | 2.657 | 6.01× |
| 1,000 | 10 | 0.814 | 3.764 | 4.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
