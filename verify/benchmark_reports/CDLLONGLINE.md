# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.59M | 0.005 | 196.02M | 0.035 | 5.02× | 6.76× |
| 10,000 | 0.115 | 86.84M | 0.102 | 98.41M | 0.170 | 1.47× | 1.67× |
| 100,000 | 1.108 | 90.22M | 1.094 | 91.40M | 1.459 | 1.32× | 1.33× |
| 1,000,000 | 11.814 | 84.64M | 11.400 | 87.72M | 14.190 | 1.20× | 1.24× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.114 ms**; native kernel **1.103 ms**; TA-Lib 1.441 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.320 | 0.258 | 3.88M | 1527.892 | 5927.69× | 106.62× |
| 100,000 | 10 | 2.514 | 1.347 | 7.42M | 1470.156 | 1091.31× | 20.75× |
| 100,000 | 1,000 | 30.013 | 26.658 | 37.51M | 1454.694 | 54.57× | 1.25× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.76M | 77.34M | 1.00× | 2.03M | 2.43M | 1.00× | 58.84M |
| 2 | 141.58M | 152.78M | 1.98× | 2.40M | 2.56M | 1.05× | 59.66M |
| 4 | 253.94M | 289.66M | 3.75× | 2.34M | 2.59M | 1.07× | 58.90M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
