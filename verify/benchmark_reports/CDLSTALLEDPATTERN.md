# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.26M | 0.010 | 99.72M | 0.042 | 3.60× | 4.16× |
| 10,000 | 0.102 | 98.38M | 0.098 | 102.12M | 0.172 | 1.69× | 1.75× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.012 ms**; TA-Lib 0.051 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.367 | 0.286 | 3.49M | 48.802 | 170.53× | 99.19× |
| 1,500 | 10 | 3.067 | 1.258 | 7.95M | 47.435 | 37.72× | 23.77× |
| 1,500 | 100 | 6.394 | 3.847 | 25.99M | 55.032 | 14.30× | 8.03× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.89M | 15.13M | 1.00× | 1.00M | 1.05M | 1.00× | 8.42M |
| 2 | 12.12M | 20.18M | 1.33× | 1.22M | 1.38M | 1.31× | 9.13M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
