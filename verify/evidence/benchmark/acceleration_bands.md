# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.78M | 0.010 | 98.49M | 0.049 | 3.54× | 4.78× |
| 10,000 | 0.093 | 107.28M | 0.085 | 117.23M | 0.116 | 1.24× | 1.36× |
| 100,000 | 1.597 | 62.63M | 1.629 | 61.40M | 1.441 | 0.90× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.147 | 1.88× |
| 1 | 5 | 0.276 | 0.589 | 2.13× |
| 1 | 10 | 0.420 | 1.042 | 2.48× |
| 10 | 1 | 0.047 | 0.108 | 2.30× |
| 10 | 5 | 0.192 | 0.521 | 2.71× |
| 10 | 10 | 0.445 | 1.075 | 2.42× |
| 100 | 1 | 0.044 | 0.110 | 2.47× |
| 100 | 5 | 0.195 | 0.516 | 2.64× |
| 100 | 10 | 0.434 | 1.121 | 2.58× |
| 1,000 | 1 | 0.064 | 0.109 | 1.70× |
| 1,000 | 5 | 0.204 | 0.565 | 2.77× |
| 1,000 | 10 | 0.419 | 1.174 | 2.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
