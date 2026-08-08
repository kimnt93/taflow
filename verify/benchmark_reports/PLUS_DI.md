# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.64M | 0.007 | 148.70M | 0.037 | 4.31× | 5.45× |
| 10,000 | 0.061 | 163.63M | 0.056 | 179.72M | 0.099 | 1.63× | 1.79× |
| 100,000 | 0.572 | 174.86M | 0.618 | 161.94M | 0.761 | 1.33× | 1.23× |
| 1,000,000 | 6.772 | 147.68M | 6.722 | 148.77M | 7.079 | 1.05× | 1.05× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.616 ms**; native kernel **0.644 ms**; TA-Lib 0.857 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.304 | 0.231 | 4.33M | 710.621 | 3077.75× | 134.64× |
| 100,000 | 10 | 1.949 | 0.970 | 10.30M | 723.253 | 745.30× | 31.92× |
| 100,000 | 1,000 | 10.147 | 7.382 | 135.46M | 697.316 | 94.46× | 5.32× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 126.34M | 132.48M | 1.00× | 2.39M | 2.71M | 1.00× | 106.07M |
| 2 | 224.09M | 253.79M | 1.92× | 2.73M | 2.32M | 0.85× | 111.37M |
| 4 | 373.57M | 484.82M | 3.66× | 2.41M | 2.77M | 1.02× | 119.31M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
