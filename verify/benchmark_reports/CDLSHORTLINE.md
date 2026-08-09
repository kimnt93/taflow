# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.57M | 0.009 | 111.62M | 0.036 | 3.33× | 3.97× |
| 10,000 | 0.129 | 77.49M | 0.143 | 70.00M | 0.207 | 1.60× | 1.45× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.011 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.362 | 0.304 | 3.29M | 42.644 | 140.34× | 113.83× |
| 1,500 | 10 | 2.663 | 1.310 | 7.64M | 40.901 | 31.23× | 23.36× |
| 1,500 | 100 | 5.918 | 3.479 | 28.74M | 41.977 | 12.07× | 9.02× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.60M | 12.60M | 1.00× | 1.24M | 980.82K | 1.00× | 7.71M |
| 2 | 17.85M | 15.05M | 1.20× | 1.31M | 1.41M | 1.44× | 8.11M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
