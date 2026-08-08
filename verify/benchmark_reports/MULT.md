# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 397.50M | 0.001 | 948.99M | 0.030 | 11.93× | 28.49× |
| 10,000 | 0.009 | 1.10G | 0.004 | 2.30G | 0.035 | 3.90× | 8.14× |
| 100,000 | 0.063 | 1.58G | 0.038 | 2.63G | 0.069 | 1.09× | 1.81× |
| 1,000,000 | 1.293 | 773.57M | 0.822 | 1.22G | 1.038 | 0.80× | 1.26× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.063 ms**; native kernel **0.039 ms**; TA-Lib 0.070 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.289 | 0.183 | 5.46M | 68.511 | 374.05× | 155.19× |
| 100,000 | 10 | 1.446 | 0.741 | 13.50M | 69.241 | 93.50× | 39.13× |
| 100,000 | 1,000 | 3.759 | 2.069 | 483.29M | 70.042 | 33.85× | 14.40× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 506.87M | 832.13M | 1.00× | 2.69M | 3.20M | 1.00× | 556.28M |
| 2 | 1.03G | 1.68G | 2.02× | 2.67M | 3.53M | 1.10× | 689.39M |
| 4 | 905.43M | 1.96G | 2.35× | 2.58M | 2.89M | 0.90× | 526.89M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
