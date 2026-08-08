# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.41M | 0.037 | 27.20M | 0.040 | 1.05× | 1.08× |
| 10,000 | 0.392 | 25.49M | 0.418 | 23.90M | 0.118 | 0.30× | 0.28× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.056 ms**; native kernel **0.058 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 1.050 | 0.321 | 3.12M | 44.110 | 137.46× | 104.00× |
| 1,500 | 10 | 2.906 | 1.493 | 6.70M | 44.894 | 30.08× | 22.94× |
| 1,500 | 100 | 8.901 | 6.400 | 15.62M | 44.056 | 6.88× | 5.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
