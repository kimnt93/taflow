# SwingHighLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.75M | 0.046 | 21.89M | nan | — | — |
| 10,000 | 0.450 | 22.23M | 0.435 | 22.99M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.072 ms**; native kernel **0.068 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.663 | 0.297 | 3.37M | nan | — | — |
| 1,500 | 10 | 1.893 | 1.115 | 8.97M | nan | — | — |
| 1,500 | 100 | 6.904 | 5.856 | 17.08M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
