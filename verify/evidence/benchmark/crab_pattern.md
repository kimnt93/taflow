# CrabPattern benchmark (`Crab` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.62M | 0.008 | 131.87M | 0.241 | 22.52× | 31.72× |
| 10,000 | 0.091 | 109.46M | 0.087 | 114.32M | 1.409 | 15.42× | 16.10× |
| 100,000 | 0.883 | 113.28M | 0.872 | 114.63M | 13.110 | 14.85× | 15.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.242 | 2.97× |
| 1 | 5 | 0.295 | 0.826 | 2.80× |
| 1 | 10 | 0.402 | 1.671 | 4.16× |
| 10 | 1 | 0.051 | 0.182 | 3.58× |
| 10 | 5 | 0.199 | 1.097 | 5.52× |
| 10 | 10 | 0.404 | 1.665 | 4.12× |
| 100 | 1 | 0.053 | 0.203 | 3.86× |
| 100 | 5 | 0.193 | 1.150 | 5.95× |
| 100 | 10 | 0.458 | 1.841 | 4.02× |
| 1,000 | 1 | 0.060 | 0.302 | 5.01× |
| 1,000 | 5 | 0.202 | 1.774 | 8.78× |
| 1,000 | 10 | 0.445 | 3.114 | 7.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
