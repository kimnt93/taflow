# FairValueGap benchmark (`smartmoneyconcepts.smc.fvg` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.22M | 0.012 | 81.80M | 3.219 | 190.66× | 263.35× |
| 10,000 | 0.112 | 89.30M | 0.100 | 100.38M | 9.195 | 82.11× | 92.30× |
| 100,000 | 1.067 | 93.74M | 0.964 | 103.69M | 74.241 | 69.59× | 76.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 2.867 | 41.44× |
| 1 | 5 | 0.342 | 14.280 | 41.77× |
| 1 | 10 | 0.396 | 28.434 | 71.75× |
| 10 | 1 | 0.045 | 2.754 | 60.95× |
| 10 | 5 | 0.217 | 14.788 | 68.01× |
| 10 | 10 | 0.429 | 28.307 | 65.99× |
| 100 | 1 | 0.076 | 2.863 | 37.60× |
| 100 | 5 | 0.210 | 14.667 | 69.97× |
| 100 | 10 | 0.428 | 30.149 | 70.41× |
| 1,000 | 1 | 0.078 | 3.456 | 44.02× |
| 1,000 | 5 | 0.268 | 18.405 | 68.58× |
| 1,000 | 10 | 0.444 | 37.267 | 83.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
