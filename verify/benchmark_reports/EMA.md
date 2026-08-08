# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 182.87M | 0.004 | 227.30M | 0.033 | 5.97× | 7.42× |
| 10,000 | 0.041 | 243.35M | 0.038 | 263.32M | 0.058 | 1.41× | 1.53× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.289 | 0.185 | 5.41M | 36.565 | 197.88× | 165.00× |
| 1,500 | 10 | 1.213 | 0.690 | 14.50M | 36.071 | 52.29× | 44.15× |
| 1,500 | 100 | 4.816 | 3.034 | 32.96M | 36.093 | 11.90× | 10.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
