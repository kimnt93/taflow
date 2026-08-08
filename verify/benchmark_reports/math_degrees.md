# MathDegrees benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.39M | 0.002 | 562.60M | nan | — | — |
| 10,000 | 0.418 | 23.90M | 0.010 | 966.85M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.063 ms**; native kernel **0.002 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.183 | 5.47M | nan | — | — |
| 1,500 | 10 | 1.671 | 0.641 | 15.60M | nan | — | — |
| 1,500 | 100 | 6.191 | 2.108 | 47.44M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
