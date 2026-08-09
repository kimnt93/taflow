# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.55M | 0.019 | 51.38M | 0.053 | 2.60× | 2.75× |
| 10,000 | 0.171 | 58.41M | 0.167 | 60.01M | 0.195 | 1.14× | 1.17× |
| 100,000 | 1.729 | 57.85M | 1.644 | 60.84M | 1.481 | 0.86× | 0.90× |
| 1,000,000 | 15.521 | 64.43M | 14.944 | 66.92M | 15.415 | 0.99× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.129 | 1.29× |
| 1 | 5 | 0.352 | 0.571 | 1.62× |
| 1 | 10 | 0.518 | 1.039 | 2.01× |
| 10 | 1 | 0.055 | 0.099 | 1.81× |
| 10 | 5 | 0.226 | 0.490 | 2.17× |
| 10 | 10 | 0.522 | 1.045 | 2.00× |
| 100 | 1 | 0.054 | 0.096 | 1.80× |
| 100 | 5 | 0.257 | 0.529 | 2.06× |
| 100 | 10 | 0.548 | 1.065 | 1.94× |
| 1,000 | 1 | 0.066 | 0.106 | 1.61× |
| 1,000 | 5 | 0.259 | 0.540 | 2.08× |
| 1,000 | 10 | 0.532 | 1.207 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
