# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.21M | 0.008 | 129.52M | 0.038 | 3.93× | 4.89× |
| 10,000 | 0.079 | 126.06M | 0.076 | 131.57M | 0.115 | 1.45× | 1.51× |
| 100,000 | 0.872 | 114.73M | 0.837 | 119.43M | 0.816 | 0.94× | 0.97× |
| 1,000,000 | 8.913 | 112.20M | 8.666 | 115.39M | 8.251 | 0.93× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.150 | 1.55× |
| 1 | 5 | 0.341 | 0.514 | 1.51× |
| 1 | 10 | 0.529 | 1.031 | 1.95× |
| 10 | 1 | 0.056 | 0.101 | 1.80× |
| 10 | 5 | 0.244 | 0.450 | 1.84× |
| 10 | 10 | 0.543 | 1.039 | 1.91× |
| 100 | 1 | 0.058 | 0.107 | 1.85× |
| 100 | 5 | 0.242 | 0.477 | 1.97× |
| 100 | 10 | 0.564 | 1.011 | 1.79× |
| 1,000 | 1 | 0.064 | 0.105 | 1.63× |
| 1,000 | 5 | 0.248 | 0.508 | 2.05× |
| 1,000 | 10 | 0.516 | 1.068 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
