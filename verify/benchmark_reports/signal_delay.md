# SignalDelay benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.55M | 0.005 | 215.24M | nan | — | — |
| 10,000 | 0.040 | 250.40M | 0.037 | 272.10M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.299 | 0.199 | 5.01M | nan | — | — |
| 1,500 | 10 | 1.205 | 0.636 | 15.73M | nan | — | — |
| 1,500 | 100 | 2.828 | 2.100 | 47.62M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
