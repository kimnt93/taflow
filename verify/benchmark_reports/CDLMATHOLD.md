# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.22M | 0.006 | 170.04M | 0.039 | 5.12× | 6.64× |
| 10,000 | 0.090 | 111.15M | 0.084 | 119.06M | 0.116 | 1.29× | 1.38× |
| 100,000 | 0.982 | 101.80M | 0.982 | 101.80M | 0.861 | 0.88× | 0.88× |
| 1,000,000 | 9.970 | 100.30M | 9.841 | 101.62M | 8.347 | 0.84× | 0.85× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.975 ms**; native kernel **0.964 ms**; TA-Lib 0.917 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.349 | 0.278 | 3.60M | 857.365 | 3084.32× | 118.01× |
| 100,000 | 10 | 2.793 | 1.515 | 6.60M | 897.793 | 592.51× | 20.75× |
| 100,000 | 1,000 | 31.079 | 28.976 | 34.51M | 862.314 | 29.76× | 1.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 89.68M | 87.18M | 1.00× | 2.28M | 2.56M | 1.00× | 102.30M |
| 2 | 177.31M | 176.14M | 2.02× | 2.35M | 2.63M | 1.03× | 101.90M |
| 4 | 294.19M | 294.87M | 3.38× | 2.28M | 2.45M | 0.96× | 99.92M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
