# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.63M | 0.007 | 139.33M | 0.028 | 3.34× | 3.92× |
| 10,000 | 0.038 | 260.28M | 0.035 | 283.64M | 0.036 | 0.93× | 1.01× |
| 100,000 | 0.339 | 295.37M | 0.312 | 320.60M | 0.091 | 0.27× | 0.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.105 | 1.16× |
| 1 | 5 | 0.270 | 0.450 | 1.66× |
| 1 | 10 | 0.475 | 0.941 | 1.98× |
| 10 | 1 | 0.061 | 0.104 | 1.69× |
| 10 | 5 | 0.253 | 0.471 | 1.86× |
| 10 | 10 | 0.480 | 0.909 | 1.89× |
| 100 | 1 | 0.052 | 0.091 | 1.74× |
| 100 | 5 | 0.247 | 0.479 | 1.93× |
| 100 | 10 | 0.523 | 0.901 | 1.72× |
| 1,000 | 1 | 0.053 | 0.084 | 1.59× |
| 1,000 | 5 | 0.256 | 0.427 | 1.67× |
| 1,000 | 10 | 0.543 | 0.998 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
