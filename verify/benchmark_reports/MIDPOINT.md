# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.87M | 0.007 | 134.08M | 0.037 | 4.42× | 4.95× |
| 10,000 | 0.083 | 120.55M | 0.080 | 124.84M | 0.100 | 1.21× | 1.25× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.324 | 0.195 | 5.12M | 40.530 | 207.35× | 168.03× |
| 1,500 | 10 | 1.358 | 0.774 | 12.92M | 38.404 | 49.61× | 42.12× |
| 1,500 | 100 | 5.044 | 3.370 | 29.67M | 39.408 | 11.69× | 9.67× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.90M | 6.51M | 1.00× | 995.23K | 1.15M | 1.00× | 5.80M |
| 2 | 17.45M | 20.76M | 3.19× | 1.28M | 1.40M | 1.22× | 8.18M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
