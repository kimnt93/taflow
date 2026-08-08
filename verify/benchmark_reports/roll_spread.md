# RollSpread benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.56M | 0.056 | 18.01M | nan | — | — |
| 10,000 | 0.554 | 18.05M | 0.567 | 17.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.093 ms**; native kernel **0.088 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.326 | 0.241 | 4.14M | nan | — | — |
| 1,500 | 10 | 1.550 | 1.028 | 9.72M | nan | — | — |
| 1,500 | 100 | 7.260 | 6.682 | 14.97M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
