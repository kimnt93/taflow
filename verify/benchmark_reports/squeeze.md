# Squeeze benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 24.01M | 0.039 | 25.62M | nan | — | — |
| 10,000 | 0.383 | 26.09M | 0.361 | 27.69M | nan | — | — |
| 100,000 | 3.715 | 26.92M | 3.599 | 27.79M | nan | — | — |
| 1,000,000 | 57.002 | 17.54M | 46.118 | 21.68M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.720 ms**; native kernel **3.598 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.338 | 0.274 | 3.66M | nan | — | — |
| 100,000 | 10 | 2.140 | 1.273 | 7.85M | nan | — | — |
| 100,000 | 1,000 | 44.019 | 38.093 | 26.25M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22.96M | 24.69M | 1.00× | 2.20M | 1.99M | 1.00× | — |
| 2 | 42.52M | 48.38M | 1.96× | 2.17M | 2.11M | 1.06× | — |
| 4 | 65.43M | 87.07M | 3.53× | 2.11M | 2.07M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
