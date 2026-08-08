# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.089 | 11.28M | 0.040 | 25.08M | 0.037 | 0.42× | 0.92× |
| 10,000 | 0.890 | 11.24M | 0.443 | 22.56M | 0.117 | 0.13× | 0.26× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.130 ms**; native kernel **0.060 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.462 | 0.291 | 3.43M | 36.473 | 125.19× | 112.65× |
| 1,500 | 10 | 3.246 | 1.510 | 6.62M | 40.710 | 26.95× | 20.74× |
| 1,500 | 100 | 12.206 | 6.896 | 14.50M | 41.543 | 6.02× | 4.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
