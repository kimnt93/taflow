# ParabolicMovingAverageStop benchmark (`pmax` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 37.80M | 0.023 | 43.65M | 3.072 | 116.14× | 134.11× |
| 10,000 | 0.192 | 52.06M | 0.189 | 52.87M | 16.683 | 86.86× | 88.21× |
| 100,000 | 1.854 | 53.94M | 1.808 | 55.32M | 162.096 | 87.44× | 89.67× |
| 1,000,000 | 19.579 | 51.07M | 18.489 | 54.09M | 1577.645 | 80.58× | 85.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.282 | 3.66× |
| 1 | 5 | 0.374 | 1.191 | 3.19× |
| 1 | 10 | 0.500 | 2.289 | 4.58× |
| 10 | 1 | 0.052 | 1.756 | 33.65× |
| 10 | 5 | 0.243 | 8.538 | 35.16× |
| 10 | 10 | 0.496 | 17.266 | 34.83× |
| 100 | 1 | 0.082 | 1.809 | 22.18× |
| 100 | 5 | 0.292 | 9.534 | 32.70× |
| 100 | 10 | 0.547 | 18.488 | 33.79× |
| 1,000 | 1 | 0.074 | 3.163 | 43.02× |
| 1,000 | 5 | 0.286 | 16.674 | 58.30× |
| 1,000 | 10 | 0.509 | 34.988 | 68.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
