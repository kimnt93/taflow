# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.56M | 0.005 | 183.09M | 0.040 | 5.76× | 7.24× |
| 10,000 | 0.050 | 199.29M | 0.044 | 224.75M | 0.091 | 1.82× | 2.05× |
| 100,000 | 0.467 | 214.24M | 0.432 | 231.42M | 0.544 | 1.17× | 1.26× |
| 1,000,000 | 5.013 | 199.48M | 4.583 | 218.21M | 5.419 | 1.08× | 1.18× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.466 ms**; native kernel **0.431 ms**; TA-Lib 0.557 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.291 | 0.188 | 5.33M | 597.789 | 3187.30× | 178.76× |
| 100,000 | 10 | 1.407 | 0.740 | 13.51M | 563.855 | 762.01× | 46.47× |
| 100,000 | 1,000 | 7.670 | 6.185 | 161.69M | 551.790 | 89.22× | 6.32× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 152.57M | 157.91M | 1.00× | 2.54M | 2.97M | 1.00× | 149.40M |
| 2 | 300.39M | 364.46M | 2.31× | 2.69M | 3.23M | 1.09× | 142.05M |
| 4 | 440.73M | 503.38M | 3.19× | 2.57M | 3.04M | 1.02× | 140.23M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
