# RollingZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.35M | 0.019 | 51.37M | nan | — | — |
| 10,000 | 0.188 | 53.11M | 0.183 | 54.53M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.028 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.289 | 0.197 | 5.07M | nan | — | — |
| 1,500 | 10 | 1.143 | 0.657 | 15.21M | nan | — | — |
| 1,500 | 100 | 4.170 | 3.367 | 29.70M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
