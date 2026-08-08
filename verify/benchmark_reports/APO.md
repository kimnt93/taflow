# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.92M | 0.012 | 85.49M | 0.038 | 0.72× | 3.25× |
| 10,000 | 0.524 | 19.07M | 0.102 | 98.52M | 0.079 | 0.15× | 0.77× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.077 ms**; native kernel **0.017 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.322 | 0.186 | 5.38M | 41.707 | 224.20× | 178.90× |
| 1,500 | 10 | 1.897 | 0.720 | 13.89M | 41.417 | 57.55× | 49.11× |
| 1,500 | 100 | 8.550 | 3.080 | 32.47M | 51.431 | 16.70× | 11.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
