# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.38M | 0.010 | 102.78M | 0.046 | 3.93× | 4.73× |
| 10,000 | 0.084 | 119.23M | 0.082 | 122.70M | 0.184 | 2.19× | 2.25× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.013 ms**; TA-Lib 0.050 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.378 | 0.280 | 3.58M | 50.835 | 181.82× | 98.81× |
| 1,500 | 10 | 4.512 | 1.276 | 7.84M | 51.496 | 40.37× | 23.20× |
| 1,500 | 100 | 6.056 | 3.585 | 27.90M | 53.846 | 15.02× | 8.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.26M | 7.24M | 1.00× | 561.17K | 1.08M | 1.00× | 8.46M |
| 2 | 18.50M | 21.80M | 3.01× | 1.34M | 1.41M | 1.31× | 8.85M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
