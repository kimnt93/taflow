# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 199.38M | 0.004 | 280.68M | 0.050 | 9.97× | 14.04× |
| 10,000 | 0.030 | 331.82M | 0.023 | 425.71M | 0.134 | 4.44× | 5.69× |
| 100,000 | 0.290 | 344.38M | 0.213 | 469.62M | 0.937 | 3.23× | 4.40× |
| 1,000,000 | 14.225 | 70.30M | 2.310 | 432.92M | 17.944 | 1.26× | 7.77× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.284 ms**; native kernel **0.214 ms**; TA-Lib 0.934 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.333 | 0.253 | 3.95M | 924.175 | 3650.49× | 159.43× |
| 100,000 | 10 | 1.745 | 1.217 | 8.22M | 944.584 | 776.09× | 35.24× |
| 100,000 | 1,000 | 94.535 | 82.272 | 12.15M | 966.426 | 11.75× | 0.61× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 189.99M | 256.76M | 1.00× | 1.85M | 1.98M | 1.00× | 80.48M |
| 2 | 263.27M | 446.38M | 1.74× | 1.80M | 1.61M | 0.81× | 78.65M |
| 4 | 271.96M | 553.88M | 2.16× | 1.36M | 1.55M | 0.78× | 80.26M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
