# ParabolicMovingAverageStop benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.94M | 0.019 | 51.94M | nan | — | — |
| 10,000 | 0.179 | 55.88M | 0.176 | 56.83M | nan | — | — |
| 100,000 | 1.744 | 57.35M | 1.691 | 59.13M | nan | — | — |
| 1,000,000 | 18.557 | 53.89M | 17.607 | 56.80M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.755 ms**; native kernel **1.685 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.391 | 0.271 | 3.69M | nan | — | — |
| 100,000 | 10 | 1.453 | 0.929 | 10.76M | nan | — | — |
| 100,000 | 1,000 | 21.974 | 19.175 | 52.15M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 47.59M | 51.78M | 1.00× | 2.13M | 2.42M | 1.00× | — |
| 2 | 47.47M | 50.08M | 0.97× | 1.98M | 2.34M | 0.96× | — |
| 4 | 48.05M | 51.79M | 1.00× | 2.00M | 2.42M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
