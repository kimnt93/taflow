# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.39M | 0.008 | 119.71M | 0.040 | 3.91× | 4.80× |
| 10,000 | 0.131 | 76.11M | 0.119 | 83.75M | 0.124 | 0.95× | 1.04× |
| 100,000 | 1.086 | 92.09M | 1.078 | 92.80M | 0.926 | 0.85× | 0.86× |
| 1,000,000 | 12.873 | 77.68M | 12.490 | 80.06M | 8.974 | 0.70× | 0.72× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.074 ms**; native kernel **1.055 ms**; TA-Lib 0.908 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.390 | 0.284 | 3.52M | 925.802 | 3262.89× | 112.66× |
| 100,000 | 10 | 3.451 | 2.688 | 3.72M | 899.867 | 334.78× | 11.80× |
| 100,000 | 1,000 | 37.961 | 33.130 | 30.18M | 924.240 | 27.90× | 1.11× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 63.44M | 79.82M | 1.00× | 2.09M | 2.28M | 1.00× | 89.16M |
| 2 | 117.29M | 156.68M | 1.96× | 2.18M | 2.63M | 1.16× | 96.30M |
| 4 | 206.72M | 221.66M | 2.78× | 1.92M | 2.23M | 0.98× | 89.99M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
