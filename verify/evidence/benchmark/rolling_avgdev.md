# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.45M | 0.021 | 47.38M | 0.051 | 2.26× | 2.41× |
| 10,000 | 0.182 | 55.07M | 0.173 | 57.71M | 0.196 | 1.08× | 1.13× |
| 100,000 | 1.817 | 55.04M | 1.807 | 55.34M | 1.540 | 0.85× | 0.85× |
| 1,000,000 | 18.547 | 53.92M | 17.152 | 58.30M | 16.032 | 0.86× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.135 | 2.06× |
| 1 | 5 | 0.288 | 0.637 | 2.21× |
| 1 | 10 | 0.529 | 0.986 | 1.87× |
| 10 | 1 | 0.053 | 0.165 | 3.09× |
| 10 | 5 | 0.274 | 0.518 | 1.89× |
| 10 | 10 | 0.546 | 1.051 | 1.93× |
| 100 | 1 | 0.054 | 0.097 | 1.79× |
| 100 | 5 | 0.256 | 0.579 | 2.26× |
| 100 | 10 | 0.593 | 1.116 | 1.88× |
| 1,000 | 1 | 0.080 | 0.117 | 1.46× |
| 1,000 | 5 | 0.278 | 0.568 | 2.04× |
| 1,000 | 10 | 0.586 | 1.318 | 2.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
