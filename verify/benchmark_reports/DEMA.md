# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.13M | 0.010 | 101.92M | 0.038 | 3.54× | 3.92× |
| 10,000 | 0.078 | 128.94M | 0.078 | 127.48M | 0.093 | 1.20× | 1.18× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.014 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.288 | 0.165 | 6.06M | 43.054 | 260.94× | 197.06× |
| 1,500 | 10 | 1.148 | 0.626 | 15.99M | 42.658 | 68.19× | 62.58× |
| 1,500 | 100 | 3.723 | 2.596 | 38.53M | 44.832 | 17.27× | 12.83× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.35M | 11.12M | 1.00× | 849.73K | 1.15M | 1.00× | 8.32M |
| 2 | 18.93M | 21.31M | 1.92× | 1.42M | 1.71M | 1.49× | 9.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
