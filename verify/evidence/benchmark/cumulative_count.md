# CumulativeCount benchmark (`one-based cumulative count` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 617.46M | 0.001 | 1.27G | 0.012 | 7.62× | 15.69× |
| 10,000 | 0.005 | 2.06G | 0.002 | 4.15G | 0.017 | 3.56× | 7.17× |
| 100,000 | 0.042 | 2.36G | 0.018 | 5.42G | 0.058 | 1.37× | 3.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.109 | 1.46× |
| 1 | 5 | 0.365 | 0.306 | 0.84× |
| 1 | 10 | 0.402 | 0.564 | 1.40× |
| 10 | 1 | 0.040 | 0.056 | 1.41× |
| 10 | 5 | 0.176 | 0.287 | 1.63× |
| 10 | 10 | 0.376 | 0.596 | 1.58× |
| 100 | 1 | 0.040 | 0.061 | 1.52× |
| 100 | 5 | 0.177 | 0.284 | 1.60× |
| 100 | 10 | 0.394 | 0.600 | 1.52× |
| 1,000 | 1 | 0.050 | 0.057 | 1.12× |
| 1,000 | 5 | 0.193 | 0.305 | 1.58× |
| 1,000 | 10 | 0.367 | 0.587 | 1.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
