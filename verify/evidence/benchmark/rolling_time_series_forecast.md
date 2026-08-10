# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.00M | 0.020 | 50.11M | 0.057 | 2.57× | 2.86× |
| 10,000 | 0.182 | 54.96M | 0.169 | 59.21M | 0.201 | 1.11× | 1.19× |
| 100,000 | 1.592 | 62.81M | 1.515 | 66.00M | 1.410 | 0.89× | 0.93× |
| 1,000,000 | 21.961 | 45.54M | 15.662 | 63.85M | 15.811 | 0.72× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.128 | 1.02× |
| 1 | 5 | 0.306 | 0.582 | 1.90× |
| 1 | 10 | 0.556 | 1.156 | 2.08× |
| 10 | 1 | 0.056 | 0.095 | 1.69× |
| 10 | 5 | 0.290 | 0.600 | 2.07× |
| 10 | 10 | 0.533 | 1.041 | 1.95× |
| 100 | 1 | 0.057 | 0.099 | 1.74× |
| 100 | 5 | 0.266 | 0.615 | 2.32× |
| 100 | 10 | 0.636 | 1.082 | 1.70× |
| 1,000 | 1 | 0.074 | 0.118 | 1.60× |
| 1,000 | 5 | 0.331 | 0.649 | 1.96× |
| 1,000 | 10 | 0.657 | 1.207 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
