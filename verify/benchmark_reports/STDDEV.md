# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.92M | 0.005 | 211.37M | 0.035 | 6.02× | 7.32× |
| 10,000 | 0.042 | 239.95M | 0.039 | 255.69M | 0.063 | 1.50× | 1.60× |
| 100,000 | 0.392 | 255.05M | 0.372 | 268.49M | 0.294 | 0.75× | 0.79× |
| 1,000,000 | 4.113 | 243.15M | 3.801 | 263.11M | 2.798 | 0.68× | 0.74× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.391 ms**; native kernel **0.374 ms**; TA-Lib 0.299 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.224 | 0.159 | 6.28M | 297.557 | 1869.47× | 194.31× |
| 100,000 | 10 | 0.960 | 0.571 | 17.52M | 290.743 | 509.32× | 56.21× |
| 100,000 | 1,000 | 6.726 | 6.288 | 159.03M | 305.098 | 48.52× | 5.47× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 186.67M | 211.20M | 1.00× | 3.21M | 3.69M | 1.00× | 216.92M |
| 2 | 360.11M | 374.04M | 1.77× | 3.12M | 3.92M | 1.06× | 245.08M |
| 4 | 503.71M | 716.83M | 3.39× | 3.15M | 3.14M | 0.85× | 239.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
