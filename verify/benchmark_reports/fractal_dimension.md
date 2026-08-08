# FractalDimension benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.125 | 8.02M | 0.124 | 8.08M | nan | — | — |
| 10,000 | 1.241 | 8.06M | 1.206 | 8.29M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.180 ms**; native kernel **0.184 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.382 | 0.293 | 3.41M | nan | — | — |
| 1,500 | 10 | 2.206 | 1.712 | 5.84M | nan | — | — |
| 1,500 | 100 | 13.523 | 12.784 | 7.82M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
