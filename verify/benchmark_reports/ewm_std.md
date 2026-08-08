# ExponentiallyWeightedStandardDeviation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.87M | 0.006 | 181.00M | nan | — | — |
| 10,000 | 0.050 | 200.43M | 0.046 | 217.40M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.269 | 0.176 | 5.67M | nan | — | — |
| 1,500 | 10 | 1.043 | 0.546 | 18.31M | nan | — | — |
| 1,500 | 100 | 2.819 | 2.064 | 48.45M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
