# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.17M | 0.008 | 127.85M | 0.032 | 3.22× | 4.06× |
| 10,000 | 0.074 | 135.57M | 0.071 | 141.30M | 0.089 | 1.21× | 1.26× |
| 100,000 | 0.739 | 135.32M | 0.698 | 143.34M | 0.650 | 0.88× | 0.93× |
| 1,000,000 | 8.704 | 114.89M | 7.371 | 135.67M | 6.481 | 0.74× | 0.88× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.715 ms**; native kernel **0.705 ms**; TA-Lib 0.654 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.256 | 0.169 | 5.91M | 644.113 | 3807.78× | 146.03× |
| 100,000 | 10 | 0.940 | 0.594 | 16.85M | 669.010 | 1127.15× | 41.49× |
| 100,000 | 1,000 | 9.331 | 8.584 | 116.50M | 664.102 | 77.37× | 3.73× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 99.70M | 102.92M | 1.00× | 2.96M | 3.51M | 1.00× | 132.00M |
| 2 | 197.12M | 214.84M | 2.09× | 2.94M | 3.40M | 0.97× | 134.59M |
| 4 | 292.47M | 379.45M | 3.69× | 2.72M | 3.10M | 0.88× | 131.19M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
