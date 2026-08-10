# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.27M | 0.019 | 53.69M | 0.093 | 5.12× | 4.97× |
| 10,000 | 0.144 | 69.66M | 0.138 | 72.21M | 0.696 | 4.85× | 5.02× |
| 100,000 | 1.446 | 69.16M | 1.396 | 71.61M | 6.449 | 4.46× | 4.62× |
| 1,000,000 | 14.482 | 69.05M | 13.938 | 71.75M | 69.128 | 4.77× | 4.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.127 | 1.62× |
| 1 | 5 | 0.285 | 0.531 | 1.86× |
| 1 | 10 | 0.602 | 1.119 | 1.86× |
| 10 | 1 | 0.058 | 0.097 | 1.68× |
| 10 | 5 | 0.276 | 0.615 | 2.22× |
| 10 | 10 | 0.610 | 1.214 | 1.99× |
| 100 | 1 | 0.054 | 0.114 | 2.09× |
| 100 | 5 | 0.287 | 0.484 | 1.68× |
| 100 | 10 | 0.537 | 1.222 | 2.28× |
| 1,000 | 1 | 0.098 | 0.163 | 1.67× |
| 1,000 | 5 | 0.287 | 0.811 | 2.83× |
| 1,000 | 10 | 0.590 | 1.824 | 3.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
