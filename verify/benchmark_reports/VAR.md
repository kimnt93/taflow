# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 181.80M | 0.004 | 228.41M | 0.035 | 6.29× | 7.91× |
| 10,000 | 0.037 | 270.30M | 0.035 | 283.04M | 0.053 | 1.44× | 1.50× |
| 100,000 | 0.351 | 284.83M | 0.339 | 294.57M | 0.230 | 0.66× | 0.68× |
| 1,000,000 | 3.728 | 268.24M | 3.416 | 292.77M | 2.123 | 0.57× | 0.62× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.353 ms**; native kernel **0.340 ms**; TA-Lib 0.229 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.214 | 0.149 | 6.70M | 228.898 | 1534.43× | 209.24× |
| 100,000 | 10 | 0.929 | 0.545 | 18.34M | 232.276 | 425.89× | 57.75× |
| 100,000 | 1,000 | 10.477 | 4.582 | 218.25M | 234.733 | 51.23× | 7.33× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 196.02M | 259.05M | 1.00× | 3.67M | 3.43M | 1.00× | 329.98M |
| 2 | 387.23M | 486.48M | 1.88× | 2.91M | 4.40M | 1.28× | 309.62M |
| 4 | 585.27M | 645.78M | 2.49× | 3.09M | 3.55M | 1.03× | 306.91M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
