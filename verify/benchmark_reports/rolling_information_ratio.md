# RollingInformationRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.51M | 0.031 | 31.95M | nan | — | — |
| 10,000 | 0.299 | 33.50M | 0.290 | 34.43M | nan | — | — |
| 100,000 | 2.967 | 33.70M | 2.975 | 33.62M | nan | — | — |
| 1,000,000 | 34.572 | 28.93M | 30.819 | 32.45M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.975 ms**; native kernel **2.968 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.279 | 0.215 | 4.64M | nan | — | — |
| 100,000 | 10 | 1.595 | 0.918 | 10.90M | nan | — | — |
| 100,000 | 1,000 | 31.144 | 29.837 | 33.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 30.28M | 29.69M | 1.00× | 2.47M | 2.17M | 1.00× | — |
| 2 | 54.92M | 59.91M | 2.02× | 2.55M | 2.81M | 1.30× | — |
| 4 | 81.13M | 72.06M | 2.43× | 2.49M | 2.76M | 1.27× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
