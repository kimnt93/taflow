# BetterVolume benchmark (`BetterVolume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.90M | 0.024 | 42.30M | 0.287 | 10.61× | 12.16× |
| 10,000 | 0.187 | 53.38M | 0.181 | 55.28M | 1.516 | 8.10× | 8.38× |
| 100,000 | 1.772 | 56.42M | 1.917 | 52.16M | 15.307 | 8.64× | 7.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.278 | 4.02× |
| 1 | 5 | 0.348 | 1.119 | 3.22× |
| 1 | 10 | 0.598 | 2.604 | 4.35× |
| 10 | 1 | 0.064 | 0.236 | 3.66× |
| 10 | 5 | 0.293 | 1.169 | 3.99× |
| 10 | 10 | 0.545 | 2.425 | 4.45× |
| 100 | 1 | 0.079 | 0.261 | 3.29× |
| 100 | 5 | 0.297 | 1.396 | 4.71× |
| 100 | 10 | 0.587 | 2.751 | 4.68× |
| 1,000 | 1 | 0.082 | 0.366 | 4.45× |
| 1,000 | 5 | 0.284 | 2.152 | 7.58× |
| 1,000 | 10 | 0.642 | 3.957 | 6.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
