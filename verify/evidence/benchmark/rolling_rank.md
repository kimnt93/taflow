# RollingRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.81M | 0.018 | 54.07M | 0.147 | 7.90× | 7.94× |
| 10,000 | 0.173 | 57.91M | 0.174 | 57.63M | 0.735 | 4.26× | 4.24× |
| 100,000 | 1.734 | 57.66M | 1.737 | 57.59M | 6.844 | 3.95× | 3.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.134 | 1.36× |
| 1 | 5 | 0.228 | 0.692 | 3.03× |
| 1 | 10 | 0.375 | 1.112 | 2.96× |
| 10 | 1 | 0.045 | 0.111 | 2.45× |
| 10 | 5 | 0.214 | 0.554 | 2.59× |
| 10 | 10 | 0.409 | 1.108 | 2.71× |
| 100 | 1 | 0.046 | 0.164 | 3.56× |
| 100 | 5 | 0.201 | 0.870 | 4.33× |
| 100 | 10 | 0.511 | 1.725 | 3.37× |
| 1,000 | 1 | 0.062 | 0.223 | 3.62× |
| 1,000 | 5 | 0.207 | 1.033 | 4.99× |
| 1,000 | 10 | 0.490 | 2.082 | 4.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
