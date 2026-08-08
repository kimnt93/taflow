# FibonacciRetracement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.190 | 5.27M | 0.189 | 5.30M | nan | — | — |
| 10,000 | 1.931 | 5.18M | 1.959 | 5.10M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.287 ms**; native kernel **0.280 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.528 | 0.421 | 2.38M | nan | — | — |
| 1,500 | 10 | 4.425 | 2.410 | 4.15M | nan | — | — |
| 1,500 | 100 | 20.621 | 20.014 | 5.00M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
