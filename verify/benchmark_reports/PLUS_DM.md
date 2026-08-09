# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.03M | 0.006 | 176.51M | 0.035 | 5.09× | 6.24× |
| 10,000 | 0.055 | 182.18M | 0.050 | 199.10M | 0.078 | 1.43× | 1.56× |
| 100,000 | 0.514 | 194.44M | 0.484 | 206.48M | 0.504 | 0.98× | 1.04× |
| 1,000,000 | 5.338 | 187.33M | 5.136 | 194.70M | 4.853 | 0.91× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.502 ms**; native kernel **0.481 ms**; TA-Lib 0.506 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.178 | 5.62M | 515.917 | 2898.53× | 168.51× |
| 100,000 | 10 | 1.349 | 0.750 | 13.34M | 502.006 | 669.61× | 39.97× |
| 100,000 | 1,000 | 7.750 | 6.403 | 156.18M | 508.760 | 79.46× | 5.50× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 139.07M | 146.83M | 1.00× | 2.40M | 3.14M | 1.00× | 147.21M |
| 2 | 271.71M | 325.91M | 2.22× | 3.19M | 3.53M | 1.12× | 157.68M |
| 4 | 439.96M | 551.79M | 3.76× | 3.10M | 3.18M | 1.01× | 159.79M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
