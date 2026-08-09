# CloseToCloseSigma benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.36M | 0.022 | 46.40M | nan | — | — |
| 10,000 | 0.194 | 51.49M | 0.192 | 52.10M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.033 ms**; native kernel **0.032 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.267 | 0.190 | 5.26M | nan | — | — |
| 1,500 | 10 | 1.187 | 0.745 | 13.41M | nan | — | — |
| 1,500 | 100 | 3.654 | 3.081 | 32.45M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.77M | 12.95M | 1.00× | 1.27M | 1.14M | 1.00× | — |
| 2 | 16.91M | 20.30M | 1.57× | 1.32M | 1.48M | 1.29× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
