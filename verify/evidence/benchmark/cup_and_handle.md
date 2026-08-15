# CupAndHandle benchmark (`CupAndHandle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.11M | 0.007 | 151.42M | 0.221 | 21.68× | 33.46× |
| 10,000 | 0.091 | 110.47M | 0.084 | 119.45M | 1.390 | 15.36× | 16.61× |
| 100,000 | 0.843 | 118.62M | 0.811 | 123.37M | 13.057 | 15.49× | 16.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.216 | 2.37× |
| 1 | 5 | 0.248 | 0.935 | 3.77× |
| 1 | 10 | 0.414 | 1.665 | 4.03× |
| 10 | 1 | 0.046 | 0.172 | 3.77× |
| 10 | 5 | 0.221 | 1.177 | 5.33× |
| 10 | 10 | 0.409 | 1.664 | 4.07× |
| 100 | 1 | 0.053 | 0.185 | 3.50× |
| 100 | 5 | 0.221 | 1.222 | 5.52× |
| 100 | 10 | 0.430 | 1.823 | 4.24× |
| 1,000 | 1 | 0.068 | 0.311 | 4.60× |
| 1,000 | 5 | 0.211 | 1.789 | 8.49× |
| 1,000 | 10 | 0.421 | 3.031 | 7.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
