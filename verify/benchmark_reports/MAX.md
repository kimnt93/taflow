# RollingMax benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 228.02M | 0.003 | 300.80M | 0.036 | 8.17× | 10.78× |
| 10,000 | 0.027 | 372.89M | 0.023 | 431.31M | 0.081 | 3.03× | 3.51× |
| 100,000 | 0.238 | 420.21M | 0.213 | 470.42M | 0.532 | 2.24× | 2.50× |
| 1,000,000 | 2.991 | 334.32M | 2.515 | 397.60M | 5.068 | 1.69× | 2.02× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.241 ms**; native kernel **0.214 ms**; TA-Lib 0.533 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.250 | 0.175 | 5.71M | 533.872 | 3047.82× | 182.53× |
| 100,000 | 10 | 0.992 | 0.596 | 16.77M | 538.073 | 902.53× | 51.90× |
| 100,000 | 1,000 | 13.295 | 19.761 | 50.60M | 535.433 | 27.10× | 1.84× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 236.84M | 298.99M | 1.00× | 3.29M | 3.75M | 1.00× | 161.67M |
| 2 | 515.67M | 637.78M | 2.13× | 2.99M | 3.67M | 0.98× | 157.72M |
| 4 | 543.68M | 827.28M | 2.77× | 3.01M | 3.35M | 0.89× | 154.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
