# JurikMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.91M | 0.020 | 49.64M | nan | — | — |
| 10,000 | 0.188 | 53.27M | 0.184 | 54.22M | nan | — | — |
| 100,000 | 1.933 | 51.74M | 1.833 | 54.55M | nan | — | — |
| 1,000,000 | 19.569 | 51.10M | 18.310 | 54.61M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.854 ms**; native kernel **1.824 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.228 | 0.167 | 5.97M | nan | — | — |
| 100,000 | 10 | 0.762 | 0.584 | 17.12M | nan | — | — |
| 100,000 | 1,000 | 20.252 | 19.599 | 51.02M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 49.88M | 51.31M | 1.00× | 2.45M | 3.45M | 1.00× | — |
| 2 | 49.03M | 50.59M | 0.99× | 2.75M | 3.05M | 0.89× | — |
| 4 | 47.48M | 49.66M | 0.97× | 2.80M | 3.26M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
