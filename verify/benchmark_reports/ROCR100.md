# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 260.49M | 0.003 | 354.46M | 0.030 | 7.68× | 10.46× |
| 10,000 | 0.022 | 451.20M | 0.020 | 499.92M | 0.039 | 1.76× | 1.95× |
| 100,000 | 0.205 | 487.23M | 0.181 | 551.09M | 0.123 | 0.60× | 0.68× |
| 1,000,000 | 2.235 | 447.50M | 1.893 | 528.12M | 1.090 | 0.49× | 0.58× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.204 ms**; native kernel **0.180 ms**; TA-Lib 0.123 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.218 | 0.171 | 5.86M | 134.707 | 789.93× | 170.44× |
| 100,000 | 10 | 0.844 | 0.517 | 19.32M | 118.551 | 229.09× | 55.12× |
| 100,000 | 1,000 | 6.945 | 3.040 | 328.98M | 120.780 | 39.73× | 10.00× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 298.48M | 419.08M | 1.00× | 3.24M | 4.08M | 1.00× | 430.87M |
| 2 | 573.46M | 641.01M | 1.53× | 3.24M | 4.14M | 1.01× | 495.46M |
| 4 | 714.68M | 1.20G | 2.87× | 3.19M | 3.47M | 0.85× | 463.56M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
