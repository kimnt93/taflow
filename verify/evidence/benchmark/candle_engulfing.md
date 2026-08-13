# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.95M | 0.045 | 22.23M | 0.029 | 0.53× | 0.65× |
| 10,000 | 0.378 | 26.45M | 0.356 | 28.05M | 0.084 | 0.22× | 0.23× |
| 100,000 | 3.431 | 29.14M | 3.412 | 29.31M | 0.570 | 0.17× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.155 | 1.19× |
| 1 | 5 | 0.444 | 0.499 | 1.12× |
| 1 | 10 | 0.640 | 0.876 | 1.37× |
| 10 | 1 | 0.067 | 0.100 | 1.48× |
| 10 | 5 | 0.297 | 0.428 | 1.44× |
| 10 | 10 | 0.641 | 0.884 | 1.38× |
| 100 | 1 | 0.067 | 0.096 | 1.43× |
| 100 | 5 | 0.298 | 0.426 | 1.43× |
| 100 | 10 | 0.662 | 0.893 | 1.35× |
| 1,000 | 1 | 0.101 | 0.097 | 0.97× |
| 1,000 | 5 | 0.324 | 0.467 | 1.44× |
| 1,000 | 10 | 0.626 | 1.025 | 1.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
