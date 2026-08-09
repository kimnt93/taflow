# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.86M | 0.006 | 157.95M | 0.038 | 5.56× | 5.98× |
| 10,000 | 0.049 | 205.14M | 0.044 | 227.31M | 0.058 | 1.18× | 1.31× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.240 | 0.172 | 5.82M | 37.454 | 217.91× | 198.86× |
| 1,500 | 10 | 0.711 | 0.613 | 16.30M | 37.634 | 61.36× | 61.62× |
| 1,500 | 100 | 4.658 | 3.636 | 27.51M | 39.606 | 10.89× | 9.37× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.26M | 12.56M | 1.00× | 1.19M | 1.48M | 1.00× | 9.10M |
| 2 | 17.43M | 16.68M | 1.33× | 1.61M | 1.59M | 1.08× | 8.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
