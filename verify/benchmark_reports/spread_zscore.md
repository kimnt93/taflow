# SpreadZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.53M | 0.085 | 11.71M | nan | — | — |
| 10,000 | 0.869 | 11.51M | 0.873 | 11.45M | nan | — | — |
| 100,000 | 8.557 | 11.69M | 8.565 | 11.68M | nan | — | — |
| 1,000,000 | 87.384 | 11.44M | 85.589 | 11.68M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **8.708 ms**; native kernel **9.851 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.325 | 0.251 | 3.98M | nan | — | — |
| 100,000 | 10 | 2.147 | 1.593 | 6.28M | nan | — | — |
| 100,000 | 1,000 | 94.817 | 86.508 | 11.56M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.64M | 10.84M | 1.00× | 2.12M | 2.36M | 1.00× | — |
| 2 | 20.96M | 21.30M | 1.96× | 2.17M | 2.22M | 0.94× | — |
| 4 | 37.47M | 40.56M | 3.74× | 2.22M | 2.32M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
