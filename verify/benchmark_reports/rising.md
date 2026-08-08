# Rising benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.46M | 0.006 | 164.85M | nan | — | — |
| 10,000 | 0.052 | 190.76M | 0.048 | 207.74M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.286 | 0.195 | 5.14M | nan | — | — |
| 1,500 | 10 | 1.118 | 0.640 | 15.62M | nan | — | — |
| 1,500 | 100 | 4.602 | 2.260 | 44.25M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
