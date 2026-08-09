# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.43M | 0.016 | 63.47M | 0.042 | 2.52× | 2.69× |
| 10,000 | 0.133 | 75.16M | 0.133 | 75.02M | 0.204 | 1.53× | 1.53× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.024 ms**; native kernel **0.022 ms**; TA-Lib 0.049 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.315 | 0.188 | 5.33M | 49.220 | 262.30× | 158.43× |
| 1,500 | 10 | 1.331 | 0.794 | 12.59M | 49.740 | 62.61× | 37.27× |
| 1,500 | 100 | 4.191 | 3.103 | 32.23M | 51.089 | 16.46× | 11.01× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.77M | 10.56M | 1.00× | 1.13M | 1.45M | 1.00× | 8.59M |
| 2 | 14.18M | 17.66M | 1.67× | 1.46M | 1.29M | 0.89× | 8.65M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
