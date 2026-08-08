# RollingSkew benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 26.79M | 0.038 | 26.59M | nan | — | — |
| 10,000 | 0.360 | 27.78M | 0.359 | 27.89M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.053 ms**; native kernel **0.054 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.332 | 0.239 | 4.19M | nan | — | — |
| 1,500 | 10 | 1.320 | 0.844 | 11.85M | nan | — | — |
| 1,500 | 100 | 5.468 | 4.807 | 20.80M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
