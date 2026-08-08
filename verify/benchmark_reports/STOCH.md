# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.208 | 4.80M | 0.060 | 16.70M | 0.053 | 0.25× | 0.89× |
| 10,000 | 2.191 | 4.56M | 0.554 | 18.04M | 0.167 | 0.08× | 0.30× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.313 ms**; native kernel **0.081 ms**; TA-Lib 0.056 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.448 | 0.336 | 2.97M | 58.396 | 173.65× | 134.34× |
| 1,500 | 10 | 4.402 | 1.764 | 5.67M | 58.604 | 33.22× | 25.22× |
| 1,500 | 100 | 20.212 | 8.166 | 12.25M | 59.908 | 7.34× | 5.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
