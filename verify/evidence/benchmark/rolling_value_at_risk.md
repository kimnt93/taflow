# RollingValueAtRisk benchmark (`ValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.629 | 613.79K | 1.632 | 612.79K | 0.322 | 0.20× | 0.20× |
| 10,000 | 16.154 | 619.05K | 16.270 | 614.63K | 1.763 | 0.11× | 0.11× |
| 100,000 | 162.919 | 613.80K | 172.047 | 581.24K | 16.202 | 0.10× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.263 | 1.46× |
| 1 | 5 | 0.397 | 1.151 | 2.90× |
| 1 | 10 | 0.627 | 2.627 | 4.19× |
| 10 | 1 | 0.077 | 0.241 | 3.13× |
| 10 | 5 | 0.302 | 1.157 | 3.83× |
| 10 | 10 | 0.632 | 2.454 | 3.88× |
| 100 | 1 | 0.221 | 0.262 | 1.19× |
| 100 | 5 | 0.372 | 1.440 | 3.87× |
| 100 | 10 | 0.727 | 2.670 | 3.67× |
| 1,000 | 1 | 1.750 | 0.412 | 0.24× |
| 1,000 | 5 | 2.351 | 2.254 | 0.96× |
| 1,000 | 10 | 3.431 | 4.252 | 1.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
