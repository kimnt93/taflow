# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.28M | 0.017 | 59.12M | 0.049 | 2.69× | 2.87× |
| 10,000 | 0.168 | 59.58M | 0.162 | 61.88M | 0.175 | 1.05× | 1.09× |
| 100,000 | 1.635 | 61.15M | 1.600 | 62.49M | 1.650 | 1.01× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.051 | 0.102 | 2.01× |
| 1 | 5 | 0.248 | 0.458 | 1.85× |
| 1 | 10 | 0.381 | 0.991 | 2.60× |
| 10 | 1 | 0.049 | 0.096 | 1.96× |
| 10 | 5 | 0.200 | 0.471 | 2.35× |
| 10 | 10 | 0.398 | 0.930 | 2.34× |
| 100 | 1 | 0.041 | 0.102 | 2.49× |
| 100 | 5 | 0.208 | 0.484 | 2.33× |
| 100 | 10 | 0.420 | 0.961 | 2.29× |
| 1,000 | 1 | 0.056 | 0.107 | 1.92× |
| 1,000 | 5 | 0.204 | 0.524 | 2.56× |
| 1,000 | 10 | 0.483 | 1.124 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
