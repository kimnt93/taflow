# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 44.16M | 0.021 | 48.24M | 0.040 | 1.78× | 1.94× |
| 10,000 | 0.216 | 46.24M | 0.213 | 47.01M | 0.091 | 0.42× | 0.43× |
| 100,000 | 2.059 | 48.56M | 2.024 | 49.42M | 0.598 | 0.29× | 0.30× |
| 1,000,000 | 21.357 | 46.82M | 20.753 | 48.19M | 5.615 | 0.26× | 0.27× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.147 ms**; native kernel **2.095 ms**; TA-Lib 0.567 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.287 | 0.189 | 5.28M | 553.875 | 2924.51× | 169.94× |
| 100,000 | 10 | 1.922 | 0.873 | 11.46M | 548.991 | 629.08× | 38.58× |
| 100,000 | 1,000 | 27.933 | 27.224 | 36.73M | 573.055 | 21.05× | 1.43× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 44.55M | 46.41M | 1.00× | 2.18M | 2.26M | 1.00× | 133.85M |
| 2 | 87.27M | 86.13M | 1.86× | 2.41M | 3.03M | 1.34× | 137.93M |
| 4 | 142.52M | 156.76M | 3.38× | 2.13M | 2.63M | 1.16× | 130.23M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
