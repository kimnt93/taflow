# HigherHigh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 210.50M | 0.003 | 293.95M | nan | — | — |
| 10,000 | 0.028 | 352.49M | 0.026 | 387.41M | nan | — | — |
| 100,000 | 0.272 | 367.08M | 0.251 | 398.15M | nan | — | — |
| 1,000,000 | 3.055 | 327.38M | 2.721 | 367.54M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.270 ms**; native kernel **0.244 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.173 | 5.78M | nan | — | — |
| 100,000 | 10 | 1.392 | 0.692 | 14.45M | nan | — | — |
| 100,000 | 1,000 | 4.939 | 4.372 | 228.71M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 201.02M | 255.17M | 1.00× | 3.39M | 3.30M | 1.00× | — |
| 2 | 403.62M | 562.18M | 2.20× | 3.33M | 3.71M | 1.13× | — |
| 4 | 558.69M | 930.19M | 3.65× | 3.26M | 3.39M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
