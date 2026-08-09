# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.70M | 0.008 | 120.68M | 0.036 | 3.56× | 4.40× |
| 10,000 | 0.080 | 125.50M | 0.075 | 133.55M | 0.143 | 1.80× | 1.91× |
| 100,000 | 1.039 | 96.22M | 0.964 | 103.69M | 1.151 | 1.11× | 1.19× |
| 1,000,000 | 9.729 | 102.78M | 9.659 | 103.53M | 11.473 | 1.18× | 1.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.162 | 1.32× |
| 1 | 5 | 0.349 | 0.483 | 1.38× |
| 1 | 10 | 0.532 | 0.973 | 1.83× |
| 10 | 1 | 0.053 | 0.093 | 1.76× |
| 10 | 5 | 0.245 | 0.444 | 1.81× |
| 10 | 10 | 0.503 | 0.934 | 1.85× |
| 100 | 1 | 0.055 | 0.092 | 1.68× |
| 100 | 5 | 0.262 | 0.456 | 1.74× |
| 100 | 10 | 0.529 | 0.925 | 1.75× |
| 1,000 | 1 | 0.064 | 0.109 | 1.71× |
| 1,000 | 5 | 0.266 | 0.516 | 1.94× |
| 1,000 | 10 | 0.528 | 1.034 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
