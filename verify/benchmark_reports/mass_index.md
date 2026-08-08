# MassIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.72M | 0.009 | 114.58M | nan | — | — |
| 10,000 | 0.080 | 124.60M | 0.076 | 131.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.012 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.339 | 0.238 | 4.21M | nan | — | — |
| 1,500 | 10 | 1.839 | 0.890 | 11.23M | nan | — | — |
| 1,500 | 100 | 4.115 | 2.801 | 35.70M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
