# CumulativeSumControlChart benchmark (`CUSUM event filter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.20M | 0.026 | 39.16M | 0.769 | 26.29× | 30.10× |
| 10,000 | 0.183 | 54.71M | 0.192 | 52.08M | 5.825 | 31.87× | 30.34× |
| 100,000 | 1.729 | 57.85M | 1.538 | 65.00M | 49.493 | 28.63× | 32.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.187 | 0.137 | 0.74× |
| 1 | 5 | 0.443 | 0.437 | 0.99× |
| 1 | 10 | 0.553 | 0.808 | 1.46× |
| 10 | 1 | 0.063 | 0.097 | 1.54× |
| 10 | 5 | 0.290 | 0.441 | 1.52× |
| 10 | 10 | 0.589 | 0.882 | 1.50× |
| 100 | 1 | 0.064 | 0.142 | 2.24× |
| 100 | 5 | 0.275 | 0.644 | 2.34× |
| 100 | 10 | 0.572 | 1.325 | 2.32× |
| 1,000 | 1 | 0.085 | 0.608 | 7.16× |
| 1,000 | 5 | 0.296 | 3.081 | 10.40× |
| 1,000 | 10 | 0.586 | 6.070 | 10.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
