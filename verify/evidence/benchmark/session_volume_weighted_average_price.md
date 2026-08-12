# SessionVolumeWeightedAveragePrice benchmark (`SessionVwap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.91M | 0.015 | 65.48M | 0.367 | 15.37× | 24.02× |
| 10,000 | 0.074 | 135.86M | 0.069 | 144.33M | 2.271 | 30.85× | 32.77× |
| 100,000 | 0.627 | 159.50M | 0.608 | 164.39M | 21.325 | 34.01× | 35.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.329 | 3.59× |
| 1 | 5 | 0.394 | 1.412 | 3.59× |
| 1 | 10 | 0.602 | 2.440 | 4.05× |
| 10 | 1 | 0.063 | 0.234 | 3.74× |
| 10 | 5 | 0.267 | 1.372 | 5.15× |
| 10 | 10 | 0.583 | 2.483 | 4.26× |
| 100 | 1 | 0.062 | 0.255 | 4.14× |
| 100 | 5 | 0.272 | 1.457 | 5.36× |
| 100 | 10 | 0.602 | 2.676 | 4.44× |
| 1,000 | 1 | 0.069 | 0.465 | 6.77× |
| 1,000 | 5 | 0.293 | 2.579 | 8.79× |
| 1,000 | 10 | 0.601 | 4.855 | 8.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
