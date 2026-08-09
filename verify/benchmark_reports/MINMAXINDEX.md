# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.76M | 0.011 | 90.73M | 0.046 | 3.72× | 4.18× |
| 10,000 | 0.107 | 93.84M | 0.101 | 99.26M | 0.148 | 1.39× | 1.47× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.015 ms**; TA-Lib 0.053 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.399 | 0.238 | 4.20M | 57.810 | 243.06× | 143.35× |
| 1,500 | 10 | 1.396 | 0.774 | 12.92M | 49.300 | 63.72× | 43.35× |
| 1,500 | 100 | 5.711 | 3.840 | 26.04M | 55.324 | 14.41× | 9.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.75M | 12.32M | 1.00× | 1.14M | 1.17M | 1.00× | 8.85M |
| 2 | 18.17M | 16.23M | 1.32× | 1.03M | 1.27M | 1.08× | 8.19M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
