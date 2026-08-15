# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.43M | 0.008 | 132.06M | 0.055 | 6.27× | 7.29× |
| 10,000 | 0.075 | 133.37M | 0.062 | 161.70M | 0.098 | 1.31× | 1.59× |
| 100,000 | 1.841 | 54.33M | 0.636 | 157.19M | 0.533 | 0.29× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.197 | 1.66× |
| 1 | 5 | 0.225 | 0.555 | 2.47× |
| 1 | 10 | 0.383 | 1.101 | 2.88× |
| 10 | 1 | 0.048 | 0.105 | 2.20× |
| 10 | 5 | 0.203 | 0.591 | 2.91× |
| 10 | 10 | 0.439 | 1.140 | 2.60× |
| 100 | 1 | 0.044 | 0.115 | 2.63× |
| 100 | 5 | 0.191 | 0.529 | 2.76× |
| 100 | 10 | 0.454 | 1.151 | 2.53× |
| 1,000 | 1 | 0.053 | 0.116 | 2.19× |
| 1,000 | 5 | 0.223 | 0.582 | 2.61× |
| 1,000 | 10 | 0.467 | 1.225 | 2.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
