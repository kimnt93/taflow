# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.04M | 0.013 | 79.24M | 0.039 | 2.82× | 3.10× |
| 10,000 | 0.104 | 96.38M | 0.108 | 92.92M | 0.126 | 1.21× | 1.17× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.019 ms**; native kernel **0.018 ms**; TA-Lib 0.046 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.326 | 0.194 | 5.14M | 45.115 | 232.02× | 160.86× |
| 1,500 | 10 | 1.297 | 0.786 | 12.72M | 44.065 | 56.05× | 37.17× |
| 1,500 | 100 | 3.969 | 2.931 | 34.12M | 46.121 | 15.74× | 10.34× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.46M | 11.55M | 1.00× | 1.30M | 1.58M | 1.00× | 9.43M |
| 2 | 15.95M | 18.92M | 1.64× | 1.37M | 1.60M | 1.01× | 10.10M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
