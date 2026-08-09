# RollingSharpe benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.15M | 0.019 | 52.22M | nan | — | — |
| 10,000 | 0.175 | 57.28M | 0.178 | 56.12M | nan | — | — |
| 100,000 | 1.724 | 58.00M | 1.713 | 58.38M | nan | — | — |
| 1,000,000 | 17.494 | 57.16M | 17.552 | 56.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.726 ms**; native kernel **1.867 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.226 | 0.177 | 5.66M | nan | — | — |
| 100,000 | 10 | 0.994 | 0.688 | 14.53M | nan | — | — |
| 100,000 | 1,000 | 19.023 | 18.111 | 55.21M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 49.49M | 53.58M | 1.00× | 3.38M | 2.70M | 1.00× | — |
| 2 | 79.55M | 77.22M | 1.44× | 2.95M | 3.15M | 1.16× | — |
| 4 | 142.31M | 147.23M | 2.75× | 2.81M | 3.04M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
