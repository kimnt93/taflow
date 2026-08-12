# ForceIndex benchmark (`ForceIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.05M | 0.008 | 132.43M | 0.205 | 21.71× | 27.11× |
| 10,000 | 0.056 | 177.42M | 0.052 | 194.08M | 0.882 | 15.64× | 17.11× |
| 100,000 | 0.449 | 222.65M | 0.432 | 231.27M | 6.833 | 15.21× | 15.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.310 | 2.63× |
| 1 | 5 | 0.347 | 1.253 | 3.61× |
| 1 | 10 | 0.485 | 2.512 | 5.18× |
| 10 | 1 | 0.054 | 0.225 | 4.17× |
| 10 | 5 | 0.254 | 1.429 | 5.62× |
| 10 | 10 | 0.514 | 2.544 | 4.95× |
| 100 | 1 | 0.062 | 0.239 | 3.84× |
| 100 | 5 | 0.242 | 1.467 | 6.07× |
| 100 | 10 | 0.561 | 2.540 | 4.53× |
| 1,000 | 1 | 0.058 | 0.290 | 5.01× |
| 1,000 | 5 | 0.253 | 1.836 | 7.26× |
| 1,000 | 10 | 0.555 | 3.276 | 5.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
