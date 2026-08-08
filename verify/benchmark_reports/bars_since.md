# BarsSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 239.07M | 0.003 | 302.93M | nan | — | — |
| 10,000 | 0.029 | 346.39M | 0.026 | 381.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.403 | 0.286 | 3.50M | nan | — | — |
| 1,500 | 10 | 0.869 | 0.549 | 18.22M | nan | — | — |
| 1,500 | 100 | 2.286 | 1.802 | 55.49M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
