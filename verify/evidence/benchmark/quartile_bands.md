# QuartileBands benchmark (`QuartileBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.03M | 0.054 | 18.51M | 0.695 | 12.52× | 12.86× |
| 10,000 | 0.614 | 16.29M | 0.587 | 17.04M | 5.615 | 9.15× | 9.57× |
| 100,000 | 6.187 | 16.16M | 6.005 | 16.65M | 68.504 | 11.07× | 11.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.298 | 4.34× |
| 1 | 5 | 0.279 | 1.195 | 4.28× |
| 1 | 10 | 0.408 | 2.559 | 6.27× |
| 10 | 1 | 0.045 | 0.224 | 4.95× |
| 10 | 5 | 0.189 | 1.073 | 5.68× |
| 10 | 10 | 0.489 | 2.425 | 4.96× |
| 100 | 1 | 0.054 | 0.289 | 5.33× |
| 100 | 5 | 0.241 | 1.641 | 6.82× |
| 100 | 10 | 0.444 | 3.165 | 7.13× |
| 1,000 | 1 | 0.113 | 0.903 | 7.98× |
| 1,000 | 5 | 0.357 | 4.596 | 12.89× |
| 1,000 | 10 | 0.662 | 9.017 | 13.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
