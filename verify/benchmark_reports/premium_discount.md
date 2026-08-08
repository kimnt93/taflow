# PremiumDiscount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.36M | 0.040 | 25.15M | nan | — | — |
| 10,000 | 0.373 | 26.83M | 0.369 | 27.09M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.058 ms**; native kernel **0.057 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.354 | 0.252 | 3.97M | nan | — | — |
| 1,500 | 10 | 1.193 | 0.879 | 11.38M | nan | — | — |
| 1,500 | 100 | 5.576 | 21.726 | 4.60M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
