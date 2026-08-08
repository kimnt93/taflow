# RollingMedian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.31M | 0.041 | 24.49M | nan | — | — |
| 10,000 | 0.464 | 21.55M | 0.438 | 22.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.065 ms**; native kernel **0.062 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.349 | 0.236 | 4.24M | nan | — | — |
| 1,500 | 10 | 1.532 | 0.993 | 10.07M | nan | — | — |
| 1,500 | 100 | 6.852 | 5.928 | 16.87M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
