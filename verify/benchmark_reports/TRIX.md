# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 171.64M | 0.005 | 199.08M | 0.039 | 6.61× | 7.67× |
| 10,000 | 0.028 | 356.26M | 0.025 | 402.67M | 0.116 | 4.15× | 4.69× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.250 | 0.175 | 5.71M | 43.378 | 247.59× | 175.93× |
| 1,500 | 10 | 1.029 | 0.585 | 17.09M | 43.375 | 74.11× | 51.02× |
| 1,500 | 100 | 2.729 | 1.765 | 56.66M | 46.610 | 26.41× | 18.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.33M | 17.18M | 1.00× | 1.30M | 1.51M | 1.00× | 8.96M |
| 2 | 13.05M | 22.17M | 1.29× | 1.64M | 1.76M | 1.17× | 9.32M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
