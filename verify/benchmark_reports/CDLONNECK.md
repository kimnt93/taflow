# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.74M | 0.004 | 237.37M | 0.033 | 5.52× | 7.90× |
| 10,000 | 0.072 | 138.23M | 0.070 | 143.25M | 0.122 | 1.68× | 1.74× |
| 100,000 | 0.866 | 115.45M | 0.850 | 117.70M | 0.994 | 1.15× | 1.17× |
| 1,000,000 | 9.487 | 105.40M | 9.445 | 105.88M | 10.060 | 1.06× | 1.07× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.901 ms**; native kernel **0.856 ms**; TA-Lib 0.989 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.369 | 0.305 | 3.27M | 1009.359 | 3303.96× | 94.59× |
| 100,000 | 10 | 2.793 | 1.426 | 7.01M | 968.803 | 679.56× | 20.35× |
| 100,000 | 1,000 | 31.953 | 28.999 | 34.48M | 966.602 | 33.33× | 1.15× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 85.33M | 97.83M | 1.00× | 2.34M | 2.15M | 1.00× | 87.50M |
| 2 | 177.49M | 188.93M | 1.93× | 2.25M | 2.52M | 1.17× | 84.17M |
| 4 | 323.09M | 338.93M | 3.46× | 2.31M | 2.43M | 1.13× | 88.55M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
