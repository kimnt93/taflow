# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.35M | 0.003 | 363.81M | 0.034 | 5.24× | 12.42× |
| 10,000 | 0.073 | 137.56M | 0.066 | 152.15M | 0.085 | 1.17× | 1.30× |
| 100,000 | 0.797 | 125.52M | 0.778 | 128.48M | 0.604 | 0.76× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.105 | 1.46× |
| 1 | 5 | 0.284 | 0.447 | 1.57× |
| 1 | 10 | 0.397 | 0.928 | 2.33× |
| 10 | 1 | 0.043 | 0.099 | 2.27× |
| 10 | 5 | 0.187 | 0.445 | 2.37× |
| 10 | 10 | 0.392 | 0.903 | 2.30× |
| 100 | 1 | 0.044 | 0.091 | 2.08× |
| 100 | 5 | 0.206 | 0.455 | 2.21× |
| 100 | 10 | 0.413 | 0.885 | 2.14× |
| 1,000 | 1 | 0.049 | 0.097 | 1.97× |
| 1,000 | 5 | 0.193 | 0.466 | 2.42× |
| 1,000 | 10 | 0.464 | 0.982 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
