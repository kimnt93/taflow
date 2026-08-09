# RollingSortino benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.96M | 0.013 | 75.82M | nan | — | — |
| 10,000 | 0.118 | 85.02M | 0.115 | 86.67M | nan | — | — |
| 100,000 | 1.166 | 85.79M | 1.138 | 87.85M | nan | — | — |
| 1,000,000 | 11.867 | 84.27M | 11.384 | 87.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.175 ms**; native kernel **1.166 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.270 | 0.309 | 3.23M | nan | — | — |
| 100,000 | 10 | 2.058 | 1.145 | 8.73M | nan | — | — |
| 100,000 | 1,000 | 15.923 | 23.663 | 42.26M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 69.56M | 79.26M | 1.00× | 3.09M | 3.32M | 1.00× | — |
| 2 | 150.33M | 154.65M | 1.95× | 3.36M | 3.52M | 1.06× | — |
| 4 | 226.59M | 289.48M | 3.65× | 3.20M | 3.26M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
