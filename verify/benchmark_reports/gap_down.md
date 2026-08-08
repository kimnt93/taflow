# GapDown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.48M | 0.004 | 249.05M | nan | — | — |
| 10,000 | 0.035 | 288.13M | 0.031 | 326.73M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.316 | 0.228 | 4.39M | nan | — | — |
| 1,500 | 10 | 1.577 | 0.764 | 13.10M | nan | — | — |
| 1,500 | 100 | 3.376 | 2.294 | 43.59M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
