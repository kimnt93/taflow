# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.46M | 0.008 | 130.21M | 0.028 | 2.72× | 3.70× |
| 10,000 | 0.059 | 169.92M | 0.054 | 184.55M | 0.074 | 1.26× | 1.37× |
| 100,000 | 0.559 | 178.86M | 0.560 | 178.51M | 0.460 | 0.82× | 0.82× |
| 1,000,000 | 6.066 | 164.84M | 5.762 | 173.56M | 5.269 | 0.87× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.101 | 1.26× |
| 1 | 5 | 0.276 | 0.468 | 1.69× |
| 1 | 10 | 0.600 | 0.911 | 1.52× |
| 10 | 1 | 0.053 | 0.091 | 1.72× |
| 10 | 5 | 0.262 | 0.428 | 1.64× |
| 10 | 10 | 0.533 | 0.917 | 1.72× |
| 100 | 1 | 0.059 | 0.094 | 1.60× |
| 100 | 5 | 0.236 | 0.417 | 1.77× |
| 100 | 10 | 0.550 | 0.922 | 1.68× |
| 1,000 | 1 | 0.065 | 0.099 | 1.52× |
| 1,000 | 5 | 0.285 | 0.471 | 1.65× |
| 1,000 | 10 | 0.576 | 0.956 | 1.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
