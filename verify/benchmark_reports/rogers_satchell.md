# RogersSatchell benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.01M | 0.030 | 33.74M | nan | — | — |
| 10,000 | 0.290 | 34.53M | 0.285 | 35.09M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.047 ms**; native kernel **0.046 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.440 | 0.324 | 3.09M | nan | — | — |
| 1,500 | 10 | 2.791 | 1.421 | 7.04M | nan | — | — |
| 1,500 | 100 | 6.757 | 5.107 | 19.58M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
