# Retracements benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.14M | 0.040 | 25.23M | nan | — | — |
| 10,000 | 0.415 | 24.12M | 0.411 | 24.35M | nan | — | — |
| 100,000 | 4.088 | 24.46M | 4.039 | 24.76M | nan | — | — |
| 1,000,000 | 46.024 | 21.73M | 40.025 | 24.98M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.121 ms**; native kernel **4.033 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.373 | 0.311 | 3.22M | nan | — | — |
| 100,000 | 10 | 2.446 | 1.534 | 6.52M | nan | — | — |
| 100,000 | 1,000 | 44.538 | 42.783 | 23.37M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22.83M | 24.79M | 1.00× | 1.95M | 1.90M | 1.00× | — |
| 2 | 42.89M | 46.67M | 1.88× | 2.08M | 2.08M | 1.10× | — |
| 4 | 71.96M | 84.15M | 3.39× | 1.90M | 2.00M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
