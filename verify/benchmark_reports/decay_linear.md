# DecayLinear benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.15M | 0.006 | 166.31M | nan | — | — |
| 10,000 | 0.471 | 21.23M | 0.052 | 191.47M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.069 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.338 | 0.203 | 4.91M | nan | — | — |
| 1,500 | 10 | 1.795 | 0.751 | 13.31M | nan | — | — |
| 1,500 | 100 | 6.670 | 2.660 | 37.59M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
