# InsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.65M | 0.004 | 238.16M | nan | — | — |
| 10,000 | 0.035 | 286.30M | 0.031 | 322.68M | nan | — | — |
| 100,000 | 0.315 | 317.14M | 0.287 | 348.62M | nan | — | — |
| 1,000,000 | 3.778 | 264.68M | 3.334 | 299.90M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.317 ms**; native kernel **0.285 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.263 | 0.199 | 5.03M | nan | — | — |
| 100,000 | 10 | 1.604 | 0.854 | 11.72M | nan | — | — |
| 100,000 | 1,000 | 5.837 | 4.876 | 205.09M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 205.84M | 273.76M | 1.00× | 3.18M | 2.83M | 1.00× | — |
| 2 | 382.20M | 497.65M | 1.82× | 2.84M | 3.61M | 1.28× | — |
| 4 | 544.43M | 970.93M | 3.55× | 3.01M | 3.44M | 1.22× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
