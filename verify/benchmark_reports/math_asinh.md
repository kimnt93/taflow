# MathAsinh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.77M | 0.013 | 78.88M | nan | — | — |
| 10,000 | 0.519 | 19.27M | 0.117 | 85.23M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.078 ms**; native kernel **0.019 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.531 | 0.188 | 5.31M | nan | — | — |
| 1,500 | 10 | 1.731 | 0.694 | 14.41M | nan | — | — |
| 1,500 | 100 | 6.773 | 2.916 | 34.29M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
