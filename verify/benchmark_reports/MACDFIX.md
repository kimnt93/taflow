# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 221.13M | 0.003 | 290.84M | 0.046 | 10.13× | 13.32× |
| 10,000 | 0.030 | 328.47M | 0.022 | 449.69M | 0.128 | 4.20× | 5.76× |
| 100,000 | 0.287 | 348.43M | 0.211 | 472.82M | 0.938 | 3.27× | 4.44× |
| 1,000,000 | 14.189 | 70.48M | 2.309 | 433.09M | 17.540 | 1.24× | 7.60× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.283 ms**; native kernel **0.212 ms**; TA-Lib 0.947 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.327 | 0.254 | 3.94M | 929.804 | 3662.50× | 139.30× |
| 100,000 | 10 | 1.391 | 1.155 | 8.66M | 955.514 | 827.34× | 30.84× |
| 100,000 | 1,000 | 100.785 | 74.635 | 13.40M | 940.962 | 12.61× | 0.63× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 166.57M | 228.06M | 1.00× | 1.68M | 1.71M | 1.00× | 81.14M |
| 2 | 243.84M | 437.48M | 1.92× | 1.68M | 1.66M | 0.97× | 78.90M |
| 4 | 284.84M | 595.01M | 2.61× | 1.29M | 1.34M | 0.79× | 79.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
