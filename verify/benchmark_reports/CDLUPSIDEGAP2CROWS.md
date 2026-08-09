# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 167.98M | 0.004 | 242.82M | 0.033 | 5.48× | 7.92× |
| 10,000 | 0.085 | 117.42M | 0.076 | 131.05M | 0.116 | 1.36× | 1.51× |
| 100,000 | 0.871 | 114.81M | 0.839 | 119.21M | 0.947 | 1.09× | 1.13× |
| 1,000,000 | 8.767 | 114.06M | 8.858 | 112.90M | 9.269 | 1.06× | 1.05× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.832 ms**; native kernel **0.855 ms**; TA-Lib 0.943 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.325 | 0.257 | 3.88M | 945.265 | 3671.98× | 108.11× |
| 100,000 | 10 | 2.466 | 1.335 | 7.49M | 929.060 | 695.78× | 21.60× |
| 100,000 | 1,000 | 27.718 | 28.912 | 34.59M | 959.567 | 33.19× | 1.10× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 94.86M | 98.20M | 1.00× | 2.35M | 2.42M | 1.00× | 90.84M |
| 2 | 193.16M | 197.86M | 2.01× | 2.44M | 2.45M | 1.01× | 88.55M |
| 4 | 349.04M | 365.35M | 3.72× | 2.36M | 2.44M | 1.01× | 88.58M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
