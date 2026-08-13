# RollingPairwiseBeta benchmark (`PairwiseBeta` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.097 | 10.32M | 0.094 | 10.63M | 0.222 | 2.29× | 2.36× |
| 10,000 | 0.837 | 11.95M | 0.800 | 12.50M | 0.944 | 1.13× | 1.18× |
| 100,000 | 8.035 | 12.45M | 7.843 | 12.75M | 7.994 | 0.99× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.208 | 0.247 | 1.19× |
| 1 | 5 | 0.447 | 1.077 | 2.41× |
| 1 | 10 | 0.632 | 2.213 | 3.50× |
| 10 | 1 | 0.070 | 0.210 | 3.00× |
| 10 | 5 | 0.297 | 1.209 | 4.07× |
| 10 | 10 | 0.614 | 2.278 | 3.71× |
| 100 | 1 | 0.074 | 0.220 | 2.96× |
| 100 | 5 | 0.300 | 1.368 | 4.55× |
| 100 | 10 | 0.657 | 2.286 | 3.48× |
| 1,000 | 1 | 0.173 | 0.296 | 1.71× |
| 1,000 | 5 | 0.313 | 1.688 | 5.39× |
| 1,000 | 10 | 0.673 | 3.092 | 4.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
