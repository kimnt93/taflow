# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.66M | 0.008 | 126.03M | 0.033 | 3.56× | 4.21× |
| 10,000 | 0.065 | 154.40M | 0.062 | 161.14M | 0.113 | 1.75× | 1.83× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.012 ms**; TA-Lib 0.047 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.541 | 0.272 | 3.67M | 36.602 | 134.38× | 102.73× |
| 1,500 | 10 | 2.539 | 1.258 | 7.95M | 35.964 | 28.60× | 22.06× |
| 1,500 | 100 | 5.453 | 3.235 | 30.91M | 36.445 | 11.27× | 8.65× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.85M | 8.89M | 1.00× | 864.39K | 1.21M | 1.00× | 8.26M |
| 2 | 17.75M | 17.89M | 2.01× | 1.07M | 1.39M | 1.15× | 9.73M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
