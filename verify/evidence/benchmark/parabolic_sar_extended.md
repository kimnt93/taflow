# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.52M | 0.013 | 74.19M | 0.055 | 3.70× | 4.06× |
| 10,000 | 0.124 | 80.86M | 0.114 | 87.91M | 0.098 | 0.79× | 0.86× |
| 100,000 | 1.167 | 85.66M | 1.164 | 85.88M | 0.741 | 0.63× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.130 | 1.21× |
| 1 | 5 | 0.276 | 0.614 | 2.22× |
| 1 | 10 | 0.481 | 1.144 | 2.38× |
| 10 | 1 | 0.054 | 0.112 | 2.06× |
| 10 | 5 | 0.238 | 0.552 | 2.32× |
| 10 | 10 | 0.502 | 1.138 | 2.27× |
| 100 | 1 | 0.054 | 0.108 | 2.01× |
| 100 | 5 | 0.249 | 0.577 | 2.32× |
| 100 | 10 | 0.510 | 1.148 | 2.25× |
| 1,000 | 1 | 0.064 | 0.113 | 1.77× |
| 1,000 | 5 | 0.232 | 0.566 | 2.44× |
| 1,000 | 10 | 0.516 | 1.284 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
