# MathAcosh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.29M | 0.011 | 89.18M | nan | — | — |
| 10,000 | 0.507 | 19.74M | 0.102 | 97.93M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.080 ms**; native kernel **0.016 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.366 | 0.184 | 5.42M | nan | — | — |
| 1,500 | 10 | 1.663 | 0.716 | 13.96M | nan | — | — |
| 1,500 | 100 | 6.667 | 2.979 | 33.57M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
