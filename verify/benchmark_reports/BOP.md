# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 240.90M | 0.002 | 511.59M | 0.027 | 6.53× | 13.86× |
| 10,000 | 0.012 | 826.33M | 0.007 | 1.38G | 0.037 | 3.08× | 5.14× |
| 100,000 | 0.083 | 1.20G | 0.057 | 1.76G | 0.125 | 1.51× | 2.21× |
| 1,000,000 | 1.829 | 546.69M | 1.470 | 680.41M | 1.718 | 0.94× | 1.17× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.081 ms**; native kernel **0.056 ms**; TA-Lib 0.127 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.313 | 0.253 | 3.96M | 125.305 | 495.77× | 98.66× |
| 100,000 | 10 | 2.476 | 1.080 | 9.26M | 123.088 | 113.95× | 23.34× |
| 100,000 | 1,000 | 5.209 | 2.666 | 375.07M | 124.921 | 46.85× | 10.28× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 351.72M | 439.25M | 1.00× | 2.47M | 2.54M | 1.00× | 453.12M |
| 2 | 858.32M | 1.27G | 2.89× | 2.43M | 2.74M | 1.08× | 430.75M |
| 4 | 819.13M | 1.59G | 3.61× | 2.44M | 2.54M | 1.00× | 451.11M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
