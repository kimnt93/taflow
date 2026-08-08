# MathRadians benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.09M | 0.002 | 521.02M | nan | — | — |
| 10,000 | 0.422 | 23.71M | 0.011 | 952.02M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.064 ms**; native kernel **0.002 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.327 | 0.180 | 5.56M | nan | — | — |
| 1,500 | 10 | 1.712 | 0.637 | 15.71M | nan | — | — |
| 1,500 | 100 | 6.379 | 2.039 | 49.04M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
