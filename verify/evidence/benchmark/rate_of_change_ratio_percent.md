# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.48M | 0.004 | 229.76M | 0.035 | 4.66× | 7.96× |
| 10,000 | 0.023 | 430.92M | 0.020 | 489.46M | 0.043 | 1.87× | 2.12× |
| 100,000 | 0.213 | 469.47M | 0.188 | 530.73M | 0.135 | 0.64× | 0.72× |
| 1,000,000 | 2.620 | 381.69M | 1.932 | 517.64M | 1.422 | 0.54× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.138 | 1.09× |
| 1 | 5 | 0.271 | 0.450 | 1.66× |
| 1 | 10 | 0.769 | 1.030 | 1.34× |
| 10 | 1 | 0.058 | 0.093 | 1.62× |
| 10 | 5 | 0.268 | 0.465 | 1.73× |
| 10 | 10 | 0.484 | 1.040 | 2.15× |
| 100 | 1 | 0.050 | 0.096 | 1.91× |
| 100 | 5 | 0.248 | 0.604 | 2.44× |
| 100 | 10 | 0.532 | 0.957 | 1.80× |
| 1,000 | 1 | 0.052 | 0.093 | 1.79× |
| 1,000 | 5 | 0.246 | 0.486 | 1.97× |
| 1,000 | 10 | 0.537 | 0.985 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
