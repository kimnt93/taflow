# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.09M | 0.011 | 87.17M | 0.038 | 3.03× | 3.34× |
| 10,000 | 0.160 | 62.39M | 0.178 | 56.19M | 0.183 | 1.14× | 1.03× |
| 100,000 | 1.618 | 61.81M | 1.551 | 64.47M | 1.544 | 0.95× | 1.00× |
| 1,000,000 | 16.741 | 59.73M | 16.577 | 60.32M | 15.037 | 0.90× | 0.91× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.589 ms**; native kernel **1.536 ms**; TA-Lib 1.509 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.245 | 0.174 | 5.75M | 1491.843 | 8579.04× | 152.57× |
| 100,000 | 10 | 1.138 | 0.709 | 14.10M | 1521.091 | 2145.30× | 37.72× |
| 100,000 | 1,000 | 18.431 | 18.225 | 54.87M | 1522.289 | 83.53× | 2.05× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 56.39M | 54.03M | 1.00× | 2.55M | 2.67M | 1.00× | 53.83M |
| 2 | 108.00M | 111.29M | 2.06× | 2.89M | 2.84M | 1.07× | 59.92M |
| 4 | 174.06M | 208.90M | 3.87× | 2.70M | 2.73M | 1.02× | 59.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
