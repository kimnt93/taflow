# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.82M | 0.018 | 54.36M | 0.040 | 1.89× | 2.19× |
| 10,000 | 0.161 | 62.09M | 0.158 | 63.46M | 0.157 | 0.98× | 1.00× |
| 100,000 | 1.643 | 60.86M | 1.559 | 64.13M | 1.321 | 0.80× | 0.85× |
| 1,000,000 | 16.826 | 59.43M | 15.789 | 63.33M | 13.299 | 0.79× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.181 | 1.47× |
| 1 | 5 | 0.332 | 0.509 | 1.53× |
| 1 | 10 | 0.648 | 1.051 | 1.62× |
| 10 | 1 | 0.066 | 0.088 | 1.33× |
| 10 | 5 | 0.272 | 0.443 | 1.63× |
| 10 | 10 | 0.560 | 0.944 | 1.69× |
| 100 | 1 | 0.060 | 0.092 | 1.53× |
| 100 | 5 | 0.259 | 0.458 | 1.77× |
| 100 | 10 | 0.570 | 0.974 | 1.71× |
| 1,000 | 1 | 0.076 | 0.100 | 1.32× |
| 1,000 | 5 | 0.303 | 0.538 | 1.78× |
| 1,000 | 10 | 0.593 | 1.105 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
