# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.68M | 0.016 | 60.81M | 0.035 | 1.88× | 2.13× |
| 10,000 | 0.139 | 71.73M | 0.135 | 74.33M | 0.129 | 0.93× | 0.96× |
| 100,000 | 1.324 | 75.53M | 1.322 | 75.65M | 0.984 | 0.74× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.118 | 0.92× |
| 1 | 5 | 0.304 | 0.469 | 1.54× |
| 1 | 10 | 0.553 | 0.968 | 1.75× |
| 10 | 1 | 0.060 | 0.106 | 1.77× |
| 10 | 5 | 0.275 | 0.506 | 1.84× |
| 10 | 10 | 0.570 | 0.949 | 1.66× |
| 100 | 1 | 0.055 | 0.099 | 1.82× |
| 100 | 5 | 0.245 | 0.440 | 1.80× |
| 100 | 10 | 0.583 | 0.977 | 1.67× |
| 1,000 | 1 | 0.079 | 0.106 | 1.35× |
| 1,000 | 5 | 0.277 | 0.510 | 1.84× |
| 1,000 | 10 | 0.575 | 1.078 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
