# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.06M | 0.007 | 139.84M | 0.034 | 3.63× | 4.74× |
| 10,000 | 0.090 | 110.62M | 0.087 | 114.43M | 0.103 | 1.13× | 1.17× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.010 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.368 | 0.281 | 3.56M | 38.153 | 135.93× | 98.36× |
| 1,500 | 10 | 2.673 | 1.225 | 8.16M | 36.995 | 30.20× | 23.40× |
| 1,500 | 100 | 5.998 | 3.558 | 28.11M | 39.482 | 11.10× | 8.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
