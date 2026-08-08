# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.55M | 0.024 | 42.38M | 0.033 | 1.30× | 1.40× |
| 10,000 | 0.255 | 39.20M | 0.259 | 38.55M | 0.122 | 0.48× | 0.47× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.040 ms**; native kernel **0.037 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.390 | 0.566 | 1.77M | 38.183 | 67.49× | 50.99× |
| 1,500 | 10 | 2.846 | 1.419 | 7.05M | 38.413 | 27.08× | 20.31× |
| 1,500 | 100 | 7.721 | 5.159 | 19.38M | 39.053 | 7.57× | 5.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
