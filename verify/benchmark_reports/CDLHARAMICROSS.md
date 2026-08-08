# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.23M | 0.028 | 35.15M | 0.037 | 1.20× | 1.31× |
| 10,000 | 0.314 | 31.82M | 0.312 | 32.04M | 0.139 | 0.44× | 0.45× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.046 ms**; native kernel **0.044 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 1.080 | 0.316 | 3.17M | 42.051 | 133.23× | 92.41× |
| 1,500 | 10 | 2.839 | 1.392 | 7.18M | 41.144 | 29.56× | 20.55× |
| 1,500 | 100 | 8.469 | 5.913 | 16.91M | 45.852 | 7.75× | 4.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
