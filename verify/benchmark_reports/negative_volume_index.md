# NegativeVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.61M | 0.006 | 164.63M | nan | — | — |
| 10,000 | 0.066 | 151.98M | 0.062 | 161.09M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.332 | 0.238 | 4.20M | nan | — | — |
| 1,500 | 10 | 1.762 | 0.828 | 12.08M | nan | — | — |
| 1,500 | 100 | 3.718 | 2.607 | 38.36M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
