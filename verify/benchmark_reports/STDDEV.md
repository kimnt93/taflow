# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.33M | 0.005 | 215.04M | 0.036 | 6.32× | 7.84× |
| 10,000 | 0.041 | 241.73M | 0.038 | 265.20M | 0.061 | 1.48× | 1.63× |
| 100,000 | 0.386 | 259.10M | 0.364 | 274.61M | 0.317 | 0.82× | 0.87× |
| 1,000,000 | 4.183 | 239.08M | 3.784 | 264.28M | 2.983 | 0.71× | 0.79× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.396 ms**; native kernel **0.354 ms**; TA-Lib 0.304 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.220 | 0.143 | 7.00M | 300.701 | 2103.79× | 233.85× |
| 100,000 | 10 | 1.093 | 0.537 | 18.63M | 302.839 | 564.20× | 61.11× |
| 100,000 | 1,000 | 6.236 | 4.914 | 203.51M | 308.285 | 62.74× | 7.27× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 190.31M | 211.45M | 1.00× | 2.77M | 3.83M | 1.00× | 222.30M |
| 2 | 345.31M | 410.04M | 1.94× | 2.83M | 3.89M | 1.02× | 243.28M |
| 4 | 537.54M | 664.67M | 3.14× | 3.06M | 3.37M | 0.88× | 238.34M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
