# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.37M | 0.021 | 47.15M | 0.041 | 1.86× | 1.93× |
| 10,000 | 0.309 | 32.36M | 0.261 | 38.37M | 0.145 | 0.47× | 0.56× |
| 100,000 | 2.584 | 38.70M | 2.558 | 39.10M | 1.139 | 0.44× | 0.45× |
| 1,000,000 | 26.726 | 37.42M | 25.541 | 39.15M | 11.021 | 0.41× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.113 | 0.81× |
| 1 | 5 | 0.423 | 0.508 | 1.20× |
| 1 | 10 | 0.513 | 0.952 | 1.86× |
| 10 | 1 | 0.051 | 0.098 | 1.94× |
| 10 | 5 | 0.233 | 0.450 | 1.93× |
| 10 | 10 | 0.490 | 0.973 | 1.98× |
| 100 | 1 | 0.052 | 0.097 | 1.86× |
| 100 | 5 | 0.223 | 0.471 | 2.11× |
| 100 | 10 | 0.488 | 1.021 | 2.09× |
| 1,000 | 1 | 0.079 | 0.109 | 1.38× |
| 1,000 | 5 | 0.254 | 0.561 | 2.21× |
| 1,000 | 10 | 0.545 | 1.095 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
