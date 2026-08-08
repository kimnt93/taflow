# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.08M | 0.008 | 128.15M | 0.039 | 0.74× | 4.94× |
| 10,000 | 0.500 | 20.00M | 0.068 | 146.84M | 0.096 | 0.19× | 1.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.074 ms**; native kernel **0.011 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.321 | 0.183 | 5.47M | 41.853 | 229.02× | 173.54× |
| 1,500 | 10 | 1.755 | 0.715 | 13.98M | 40.584 | 56.73× | 43.71× |
| 1,500 | 100 | 8.827 | 2.878 | 34.75M | 41.392 | 14.38× | 10.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
