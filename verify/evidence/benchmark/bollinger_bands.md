# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.07M | 0.052 | 19.19M | 0.053 | 0.80× | 1.02× |
| 10,000 | 0.429 | 23.29M | 0.385 | 25.99M | 0.095 | 0.22× | 0.25× |
| 100,000 | 5.254 | 19.03M | 4.273 | 23.40M | 0.788 | 0.15× | 0.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.123 | 1.24× |
| 1 | 5 | 0.487 | 0.619 | 1.27× |
| 1 | 10 | 0.782 | 1.146 | 1.47× |
| 10 | 1 | 0.071 | 0.117 | 1.64× |
| 10 | 5 | 0.345 | 0.537 | 1.55× |
| 10 | 10 | 0.680 | 1.128 | 1.66× |
| 100 | 1 | 0.077 | 0.112 | 1.45× |
| 100 | 5 | 0.337 | 0.555 | 1.64× |
| 100 | 10 | 0.716 | 1.132 | 1.58× |
| 1,000 | 1 | 0.110 | 0.118 | 1.07× |
| 1,000 | 5 | 0.333 | 0.561 | 1.68× |
| 1,000 | 10 | 0.698 | 1.194 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
