# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.98M | 0.023 | 43.13M | 0.035 | 1.40× | 1.51× |
| 10,000 | 0.259 | 38.54M | 0.259 | 38.58M | 0.123 | 0.47× | 0.48× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.037 ms**; native kernel **0.034 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.382 | 0.298 | 3.35M | 39.421 | 132.23× | 98.47× |
| 1,500 | 10 | 2.746 | 1.374 | 7.28M | 39.123 | 28.48× | 20.73× |
| 1,500 | 100 | 10.571 | 5.060 | 19.76M | 39.960 | 7.90× | 5.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
