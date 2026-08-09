# SwingHighLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.22M | 0.033 | 30.14M | nan | — | — |
| 10,000 | 0.394 | 25.36M | 0.363 | 27.55M | nan | — | — |
| 100,000 | 3.718 | 26.89M | 3.515 | 28.45M | nan | — | — |
| 1,000,000 | 49.067 | 20.38M | 35.699 | 28.01M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.774 ms**; native kernel **3.495 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.319 | 0.270 | 3.70M | nan | — | — |
| 100,000 | 10 | 1.828 | 1.077 | 9.29M | nan | — | — |
| 100,000 | 1,000 | 41.388 | 36.173 | 27.64M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 24.26M | 26.29M | 1.00× | 2.53M | 2.49M | 1.00× | — |
| 2 | 44.95M | 50.18M | 1.91× | 2.30M | 2.46M | 0.99× | — |
| 4 | 75.01M | 93.42M | 3.55× | 2.23M | 2.32M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
