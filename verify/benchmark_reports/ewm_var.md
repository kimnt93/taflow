# ExponentiallyWeightedVariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.34M | 0.005 | 188.84M | nan | — | — |
| 10,000 | 0.048 | 208.94M | 0.046 | 217.00M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.257 | 0.171 | 5.86M | nan | — | — |
| 1,500 | 10 | 1.011 | 0.524 | 19.10M | nan | — | — |
| 1,500 | 100 | 2.665 | 1.894 | 52.80M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
