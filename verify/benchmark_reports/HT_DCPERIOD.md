# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.55M | 0.047 | 21.29M | 0.076 | 1.56× | 1.61× |
| 10,000 | 0.474 | 21.10M | 0.465 | 21.49M | 0.500 | 1.05× | 1.07× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.070 ms**; native kernel **0.069 ms**; TA-Lib 0.094 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.333 | 0.218 | 4.58M | 93.335 | 427.73× | 131.04× |
| 1,500 | 10 | 1.625 | 1.206 | 8.29M | 99.831 | 82.76× | 25.21× |
| 1,500 | 100 | 9.310 | 7.082 | 14.12M | 107.767 | 15.22× | 4.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
