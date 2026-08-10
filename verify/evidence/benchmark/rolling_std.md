# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.52M | 0.007 | 145.87M | 0.038 | 5.15× | 5.54× |
| 10,000 | 0.048 | 208.80M | 0.045 | 220.04M | 0.065 | 1.35× | 1.42× |
| 100,000 | 0.458 | 218.26M | 0.428 | 233.87M | 0.321 | 0.70× | 0.75× |
| 1,000,000 | 4.895 | 204.30M | 4.249 | 235.32M | 3.161 | 0.65× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.132 | 1.38× |
| 1 | 5 | 0.333 | 0.578 | 1.74× |
| 1 | 10 | 0.505 | 0.984 | 1.95× |
| 10 | 1 | 0.053 | 0.105 | 1.99× |
| 10 | 5 | 0.239 | 0.445 | 1.86× |
| 10 | 10 | 0.553 | 1.002 | 1.81× |
| 100 | 1 | 0.052 | 0.097 | 1.86× |
| 100 | 5 | 0.248 | 0.487 | 1.96× |
| 100 | 10 | 0.516 | 1.167 | 2.26× |
| 1,000 | 1 | 0.057 | 0.101 | 1.76× |
| 1,000 | 5 | 0.251 | 0.462 | 1.84× |
| 1,000 | 10 | 0.506 | 1.043 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
