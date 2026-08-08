# Retracements benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.70M | 0.054 | 18.59M | nan | — | — |
| 10,000 | 0.517 | 19.33M | 0.495 | 20.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.080 ms**; native kernel **0.076 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 1.177 | 0.575 | 1.74M | nan | — | — |
| 1,500 | 10 | 4.194 | 1.458 | 6.86M | nan | — | — |
| 1,500 | 100 | 8.797 | 28.194 | 3.55M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
