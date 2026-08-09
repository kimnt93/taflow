# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.15M | 0.009 | 116.02M | 0.037 | 3.57× | 4.26× |
| 10,000 | 0.085 | 117.07M | 0.089 | 112.07M | 0.152 | 1.78× | 1.71× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.010 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.360 | 0.301 | 3.32M | 41.550 | 137.95× | 99.30× |
| 1,500 | 10 | 2.630 | 1.307 | 7.65M | 55.361 | 42.37× | 23.98× |
| 1,500 | 100 | 5.716 | 3.435 | 29.11M | 43.553 | 12.68× | 9.12× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.57M | 15.77M | 1.00× | 1.08M | 1.23M | 1.00× | 7.74M |
| 2 | 18.42M | 18.03M | 1.14× | 1.32M | 1.30M | 1.05× | 9.21M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
