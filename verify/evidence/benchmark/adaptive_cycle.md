# AdaptiveCycle benchmark (`AdaptiveCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.080 | 12.42M | 0.080 | 12.57M | 0.182 | 2.26× | 2.28× |
| 10,000 | 0.789 | 12.67M | 0.752 | 13.31M | 0.976 | 1.24× | 1.30× |
| 100,000 | 7.556 | 13.23M | 7.697 | 12.99M | 9.815 | 1.30× | 1.28× |
| 1,000,000 | 75.414 | 13.26M | 73.434 | 13.62M | 96.645 | 1.28× | 1.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.232 | 2.61× |
| 1 | 5 | 0.307 | 0.795 | 2.59× |
| 1 | 10 | 0.446 | 1.770 | 3.97× |
| 10 | 1 | 0.052 | 0.158 | 3.07× |
| 10 | 5 | 0.222 | 0.778 | 3.50× |
| 10 | 10 | 0.472 | 1.820 | 3.86× |
| 100 | 1 | 0.057 | 0.170 | 2.99× |
| 100 | 5 | 0.225 | 0.839 | 3.73× |
| 100 | 10 | 0.483 | 1.700 | 3.52× |
| 1,000 | 1 | 0.119 | 0.283 | 2.37× |
| 1,000 | 5 | 0.275 | 1.731 | 6.30× |
| 1,000 | 10 | 0.589 | 2.802 | 4.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
