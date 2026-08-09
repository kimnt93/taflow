# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.48M | 0.009 | 116.94M | 0.032 | 3.35× | 3.79× |
| 10,000 | 0.069 | 144.43M | 0.067 | 149.24M | 0.083 | 1.19× | 1.23× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.297 | 0.170 | 5.88M | 33.940 | 199.48× | 145.21× |
| 1,500 | 10 | 1.167 | 0.646 | 15.48M | 34.651 | 53.65× | 38.34× |
| 1,500 | 100 | 3.406 | 2.300 | 43.48M | 34.984 | 15.21× | 11.32× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.47M | 17.31M | 1.00× | 1.22M | 1.60M | 1.00× | 7.43M |
| 2 | 17.46M | 15.34M | 0.89× | 1.22M | 1.26M | 0.79× | 10.32M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
