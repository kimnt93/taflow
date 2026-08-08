# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.78M | 0.009 | 114.37M | 0.042 | 3.94× | 4.75× |
| 10,000 | 0.120 | 83.06M | 0.112 | 89.18M | 0.142 | 1.18× | 1.27× |
| 100,000 | 1.197 | 83.52M | 1.113 | 89.87M | 1.128 | 0.94× | 1.01× |
| 1,000,000 | 17.099 | 58.48M | 11.843 | 84.44M | 10.780 | 0.63× | 0.91× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.164 ms**; native kernel **1.128 ms**; TA-Lib 1.116 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.393 | 0.286 | 3.49M | 1096.321 | 3830.18× | 113.39× |
| 100,000 | 10 | 2.327 | 1.385 | 7.22M | 1112.317 | 802.88× | 23.48× |
| 100,000 | 1,000 | 80.333 | 80.150 | 12.48M | 1111.963 | 13.87× | 0.51× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 72.00M | 76.23M | 1.00× | 2.05M | 2.07M | 1.00× | 77.05M |
| 2 | 122.85M | 143.39M | 1.88× | 1.73M | 2.00M | 0.97× | 75.03M |
| 4 | 179.98M | 252.82M | 3.32× | 1.72M | 1.88M | 0.91× | 76.04M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
