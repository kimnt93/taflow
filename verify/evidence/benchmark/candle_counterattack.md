# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.67M | 0.017 | 59.80M | 0.033 | 1.69× | 2.00× |
| 10,000 | 0.169 | 59.33M | 0.167 | 59.75M | 0.139 | 0.83× | 0.83× |
| 100,000 | 1.808 | 55.31M | 1.847 | 54.16M | 1.161 | 0.64× | 0.63× |
| 1,000,000 | 18.034 | 55.45M | 17.416 | 57.42M | 11.503 | 0.64× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.131 | 1.70× |
| 1 | 5 | 0.310 | 0.437 | 1.41× |
| 1 | 10 | 0.509 | 1.049 | 2.06× |
| 10 | 1 | 0.075 | 0.115 | 1.53× |
| 10 | 5 | 0.253 | 0.440 | 1.74× |
| 10 | 10 | 0.539 | 0.933 | 1.73× |
| 100 | 1 | 0.058 | 0.095 | 1.65× |
| 100 | 5 | 0.294 | 0.475 | 1.62× |
| 100 | 10 | 0.541 | 0.935 | 1.73× |
| 1,000 | 1 | 0.082 | 0.109 | 1.33× |
| 1,000 | 5 | 0.285 | 0.564 | 1.98× |
| 1,000 | 10 | 0.601 | 1.049 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
