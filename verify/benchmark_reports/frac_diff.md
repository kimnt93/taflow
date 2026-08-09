# FracDiff benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.077 | 13.02M | 0.080 | 12.47M | nan | — | — |
| 10,000 | 7.424 | 1.35M | 7.105 | 1.41M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.478 ms**; native kernel **0.459 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 1.014 | 0.958 | 1.04M | nan | — | — |
| 1,500 | 10 | 8.730 | 8.365 | 1.20M | nan | — | — |
| 1,500 | 100 | 79.204 | 78.579 | 1.27M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 3.92M | 4.02M | 1.00× | 484.19K | 699.58K | 1.00× | — |
| 2 | 5.29M | 8.26M | 2.06× | 542.01K | 713.49K | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
