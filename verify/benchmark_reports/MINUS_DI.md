# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.93M | 0.011 | 94.77M | 0.038 | 3.21× | 3.63× |
| 10,000 | 0.098 | 102.52M | 0.093 | 107.95M | 0.094 | 0.97× | 1.02× |
| 100,000 | 0.946 | 105.69M | 0.915 | 109.29M | 0.655 | 0.69× | 0.72× |
| 1,000,000 | 9.964 | 100.36M | 9.571 | 104.48M | 6.486 | 0.65× | 0.68× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.934 ms**; native kernel **0.916 ms**; TA-Lib 0.673 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.270 | 0.215 | 4.66M | 662.271 | 3086.03× | 140.31× |
| 100,000 | 10 | 1.894 | 0.964 | 10.37M | 656.034 | 680.47× | 32.24× |
| 100,000 | 1,000 | 13.875 | 11.350 | 88.11M | 663.011 | 58.42× | 3.36× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 86.54M | 98.65M | 1.00× | 2.77M | 3.02M | 1.00× | 127.27M |
| 2 | 170.69M | 184.75M | 1.87× | 2.72M | 2.83M | 0.94× | 122.43M |
| 4 | 287.00M | 335.01M | 3.40× | 2.48M | 2.48M | 0.82× | 119.03M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
