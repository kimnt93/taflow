# ValueWhen benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 236.96M | 0.003 | 339.33M | nan | — | — |
| 10,000 | 0.025 | 401.23M | 0.022 | 462.49M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.004 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.463 | 0.330 | 3.03M | nan | — | — |
| 1,500 | 10 | 1.489 | 0.825 | 12.12M | nan | — | — |
| 1,500 | 100 | 3.153 | 2.267 | 44.11M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
