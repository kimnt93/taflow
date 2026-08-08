# PositiveVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.40M | 0.004 | 226.86M | nan | — | — |
| 10,000 | 0.063 | 159.69M | 0.059 | 169.82M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.334 | 0.233 | 4.29M | nan | — | — |
| 1,500 | 10 | 1.724 | 0.857 | 11.67M | nan | — | — |
| 1,500 | 100 | 4.126 | 2.620 | 38.17M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
