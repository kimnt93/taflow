# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 161.54M | 0.005 | 183.75M | 0.033 | 5.30× | 6.03× |
| 10,000 | 0.037 | 272.13M | 0.034 | 294.86M | 0.050 | 1.37× | 1.49× |
| 100,000 | 0.345 | 289.52M | 0.316 | 316.03M | 0.230 | 0.67× | 0.73× |
| 1,000,000 | 3.581 | 279.26M | 3.227 | 309.87M | 2.065 | 0.58× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.213 | 0.107 | 0.50× |
| 1 | 5 | 0.254 | 0.506 | 1.99× |
| 1 | 10 | 0.471 | 0.956 | 2.03× |
| 10 | 1 | 0.046 | 0.090 | 1.97× |
| 10 | 5 | 0.214 | 0.429 | 2.00× |
| 10 | 10 | 0.461 | 0.919 | 1.99× |
| 100 | 1 | 0.047 | 0.091 | 1.94× |
| 100 | 5 | 0.229 | 0.443 | 1.93× |
| 100 | 10 | 0.433 | 0.925 | 2.14× |
| 1,000 | 1 | 0.054 | 0.096 | 1.79× |
| 1,000 | 5 | 0.243 | 0.453 | 1.86× |
| 1,000 | 10 | 0.472 | 1.024 | 2.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
