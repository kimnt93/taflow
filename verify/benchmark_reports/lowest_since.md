# LowestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 172.98M | 0.005 | 203.56M | nan | — | — |
| 10,000 | 0.044 | 227.27M | 0.037 | 268.52M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.458 | 0.310 | 3.22M | nan | — | — |
| 1,500 | 10 | 1.417 | 0.761 | 13.14M | nan | — | — |
| 1,500 | 100 | 3.008 | 2.171 | 46.06M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
