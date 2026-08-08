# Crossunder benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.37M | 0.004 | 234.13M | nan | — | — |
| 10,000 | 0.039 | 255.78M | 0.035 | 284.17M | nan | — | — |
| 100,000 | 0.352 | 283.92M | 0.326 | 306.82M | nan | — | — |
| 1,000,000 | 4.164 | 240.18M | 3.724 | 268.50M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.363 ms**; native kernel **0.316 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.191 | 5.22M | nan | — | — |
| 100,000 | 10 | 1.457 | 0.747 | 13.39M | nan | — | — |
| 100,000 | 1,000 | 5.540 | 4.541 | 220.23M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 226.36M | 263.92M | 1.00× | 3.30M | 3.25M | 1.00× | — |
| 2 | 381.69M | 436.09M | 1.65× | 3.44M | 3.18M | 0.98× | — |
| 4 | 277.38M | 348.13M | 1.32× | 3.08M | 3.23M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
