# PivotPoints benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.00M | 0.012 | 84.51M | nan | — | — |
| 10,000 | 0.108 | 92.92M | 0.098 | 101.83M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.017 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.547 | 0.487 | 2.05M | nan | — | — |
| 1,500 | 10 | 2.034 | 1.276 | 7.84M | nan | — | — |
| 1,500 | 100 | 4.358 | 3.294 | 30.36M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
