# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.26M | 0.032 | 31.48M | 0.040 | 1.80× | 1.25× |
| 10,000 | 0.195 | 51.35M | 0.195 | 51.25M | 0.115 | 0.59× | 0.59× |
| 100,000 | 2.049 | 48.81M | 1.985 | 50.37M | 0.846 | 0.41× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.119 | 0.99× |
| 1 | 5 | 0.244 | 0.557 | 2.28× |
| 1 | 10 | 0.422 | 0.945 | 2.24× |
| 10 | 1 | 0.041 | 0.094 | 2.30× |
| 10 | 5 | 0.175 | 0.483 | 2.75× |
| 10 | 10 | 0.425 | 1.029 | 2.42× |
| 100 | 1 | 0.050 | 0.097 | 1.96× |
| 100 | 5 | 0.200 | 0.458 | 2.29× |
| 100 | 10 | 0.414 | 1.008 | 2.43× |
| 1,000 | 1 | 0.066 | 0.108 | 1.63× |
| 1,000 | 5 | 0.205 | 0.524 | 2.56× |
| 1,000 | 10 | 0.447 | 1.053 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
