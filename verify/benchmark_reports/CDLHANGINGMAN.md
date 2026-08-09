# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.52M | 0.005 | 189.56M | 0.039 | 5.52× | 7.34× |
| 10,000 | 0.121 | 82.46M | 0.115 | 87.05M | 0.168 | 1.39× | 1.47× |
| 100,000 | 1.229 | 81.37M | 1.225 | 81.64M | 1.412 | 1.15× | 1.15× |
| 1,000,000 | 12.786 | 78.21M | 12.608 | 79.31M | 14.049 | 1.10× | 1.11× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.239 ms**; native kernel **1.220 ms**; TA-Lib 1.408 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.327 | 0.270 | 3.71M | 1395.255 | 5174.28× | 102.97× |
| 100,000 | 10 | 2.621 | 1.324 | 7.55M | 1411.775 | 1066.32× | 20.71× |
| 100,000 | 1,000 | 29.375 | 25.393 | 39.38M | 1440.220 | 56.72× | 1.53× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 70.92M | 69.32M | 1.00× | 2.26M | 2.33M | 1.00× | 60.77M |
| 2 | 136.23M | 145.91M | 2.10× | 2.27M | 2.55M | 1.10× | 61.61M |
| 4 | 249.81M | 262.39M | 3.79× | 2.19M | 2.42M | 1.04× | 62.41M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
