# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.88M | 0.023 | 43.87M | 0.044 | 1.87× | 1.92× |
| 10,000 | 0.279 | 35.84M | 0.270 | 36.99M | 0.146 | 0.52× | 0.54× |
| 100,000 | 2.712 | 36.87M | 2.612 | 38.28M | 1.171 | 0.43× | 0.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.112 | 1.67× |
| 1 | 5 | 0.285 | 0.490 | 1.72× |
| 1 | 10 | 0.517 | 1.048 | 2.03× |
| 10 | 1 | 0.052 | 0.092 | 1.77× |
| 10 | 5 | 0.232 | 0.455 | 1.96× |
| 10 | 10 | 0.480 | 0.964 | 2.01× |
| 100 | 1 | 0.049 | 0.096 | 1.97× |
| 100 | 5 | 0.226 | 0.451 | 2.00× |
| 100 | 10 | 0.518 | 1.007 | 1.94× |
| 1,000 | 1 | 0.074 | 0.109 | 1.47× |
| 1,000 | 5 | 0.243 | 0.528 | 2.17× |
| 1,000 | 10 | 0.526 | 1.112 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
