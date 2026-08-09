# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.25M | 0.011 | 94.76M | 0.038 | 3.27× | 3.59× |
| 10,000 | 0.148 | 67.48M | 0.142 | 70.23M | 0.168 | 1.13× | 1.18× |
| 100,000 | 1.458 | 68.59M | 1.450 | 68.96M | 1.445 | 0.99× | 1.00× |
| 1,000,000 | 15.784 | 63.35M | 15.039 | 66.50M | 14.178 | 0.90× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.456 ms**; native kernel **1.435 ms**; TA-Lib 1.419 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.225 | 0.184 | 5.44M | 1443.295 | 7857.64× | 146.09× |
| 100,000 | 10 | 1.143 | 0.640 | 15.61M | 1465.325 | 2287.90× | 41.39× |
| 100,000 | 1,000 | 17.328 | 15.924 | 62.80M | 1423.257 | 89.38× | 2.30× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 58.27M | 59.31M | 1.00× | 2.69M | 2.75M | 1.00× | 58.91M |
| 2 | 112.15M | 114.03M | 1.92× | 2.55M | 2.86M | 1.04× | 57.61M |
| 4 | 179.20M | 209.08M | 3.53× | 2.53M | 2.69M | 0.98× | 58.61M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
