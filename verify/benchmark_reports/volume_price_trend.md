# VolumePriceTrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 187.11M | 0.004 | 257.25M | nan | — | — |
| 10,000 | 0.035 | 286.10M | 0.031 | 321.38M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.324 | 0.223 | 4.48M | nan | — | — |
| 1,500 | 10 | 1.645 | 0.774 | 12.91M | nan | — | — |
| 1,500 | 100 | 3.178 | 2.201 | 45.43M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
