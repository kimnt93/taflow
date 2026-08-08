# HeikinAshi benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.79M | 0.013 | 77.61M | nan | — | — |
| 10,000 | 0.130 | 76.76M | 0.118 | 84.90M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.021 ms**; native kernel **0.019 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.514 | 0.334 | 2.99M | nan | — | — |
| 1,500 | 10 | 1.877 | 1.675 | 5.97M | nan | — | — |
| 1,500 | 100 | 4.087 | 3.295 | 30.35M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
