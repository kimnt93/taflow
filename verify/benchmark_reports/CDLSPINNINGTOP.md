# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 154.26M | 0.005 | 201.39M | 0.036 | 5.48× | 7.15× |
| 10,000 | 0.094 | 106.66M | 0.087 | 114.44M | 0.126 | 1.35× | 1.45× |
| 100,000 | 0.992 | 100.84M | 0.974 | 102.69M | 1.027 | 1.04× | 1.05× |
| 1,000,000 | 10.151 | 98.51M | 10.070 | 99.30M | 10.404 | 1.02× | 1.03× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.980 ms**; native kernel **1.052 ms**; TA-Lib 1.122 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.365 | 0.288 | 3.47M | 1038.314 | 3602.41× | 101.48× |
| 100,000 | 10 | 2.747 | 1.546 | 6.47M | 1039.726 | 672.52× | 19.04× |
| 100,000 | 1,000 | 27.978 | 24.149 | 41.41M | 1029.941 | 42.65× | 1.40× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 86.00M | 87.60M | 1.00× | 2.23M | 2.62M | 1.00× | 87.14M |
| 2 | 167.63M | 177.04M | 2.02× | 2.37M | 2.46M | 0.94× | 82.69M |
| 4 | 295.67M | 336.73M | 3.84× | 2.27M | 2.43M | 0.93× | 82.05M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
