# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.19M | 0.019 | 51.53M | 0.040 | 1.65× | 2.06× |
| 10,000 | 0.171 | 58.56M | 0.163 | 61.45M | 0.137 | 0.80× | 0.84× |
| 100,000 | 1.685 | 59.34M | 1.622 | 61.65M | 1.067 | 0.63× | 0.66× |
| 1,000,000 | 16.604 | 60.23M | 16.328 | 61.25M | 10.641 | 0.64× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.138 | 1.28× |
| 1 | 5 | 0.344 | 0.496 | 1.44× |
| 1 | 10 | 0.602 | 1.071 | 1.78× |
| 10 | 1 | 0.062 | 0.098 | 1.58× |
| 10 | 5 | 0.295 | 0.479 | 1.62× |
| 10 | 10 | 0.578 | 1.078 | 1.86× |
| 100 | 1 | 0.078 | 0.108 | 1.39× |
| 100 | 5 | 0.305 | 0.543 | 1.78× |
| 100 | 10 | 0.590 | 1.018 | 1.73× |
| 1,000 | 1 | 0.074 | 0.123 | 1.67× |
| 1,000 | 5 | 0.306 | 0.546 | 1.79× |
| 1,000 | 10 | 0.617 | 1.143 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
