# Crossover benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 133.12M | 0.006 | 160.64M | nan | — | — |
| 10,000 | 0.036 | 280.37M | 0.033 | 301.30M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.376 | 0.201 | 4.98M | nan | — | — |
| 1,500 | 10 | 1.478 | 0.766 | 13.06M | nan | — | — |
| 1,500 | 100 | 2.502 | 1.678 | 59.59M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.85M | 9.39M | 1.00× | 1.17M | 934.77K | 1.00× | — |
| 2 | 19.08M | 20.77M | 2.21× | 1.48M | 1.59M | 1.70× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
