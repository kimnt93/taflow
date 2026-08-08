# OutsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 191.63M | 0.004 | 250.82M | nan | — | — |
| 10,000 | 0.035 | 288.24M | 0.031 | 321.14M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.326 | 0.225 | 4.44M | nan | — | — |
| 1,500 | 10 | 1.605 | 0.769 | 13.00M | nan | — | — |
| 1,500 | 100 | 3.238 | 2.207 | 45.31M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
