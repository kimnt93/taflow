# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.12M | 0.005 | 212.73M | 0.034 | 4.26× | 7.30× |
| 10,000 | 0.063 | 157.91M | 0.058 | 172.39M | 0.098 | 1.55× | 1.69× |
| 100,000 | 0.718 | 139.31M | 0.690 | 144.90M | 0.742 | 1.03× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.143 | 0.126 | 0.88× |
| 1 | 5 | 0.256 | 0.464 | 1.81× |
| 1 | 10 | 0.431 | 0.910 | 2.11× |
| 10 | 1 | 0.045 | 0.089 | 2.00× |
| 10 | 5 | 0.202 | 0.471 | 2.33× |
| 10 | 10 | 0.376 | 0.961 | 2.55× |
| 100 | 1 | 0.047 | 0.093 | 1.96× |
| 100 | 5 | 0.190 | 0.440 | 2.31× |
| 100 | 10 | 0.394 | 0.882 | 2.24× |
| 1,000 | 1 | 0.047 | 0.095 | 2.02× |
| 1,000 | 5 | 0.193 | 0.499 | 2.58× |
| 1,000 | 10 | 0.398 | 0.960 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
