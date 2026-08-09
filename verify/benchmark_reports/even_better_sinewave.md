# EvenBetterSinewave benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.21M | 0.019 | 51.89M | nan | — | — |
| 10,000 | 0.191 | 52.28M | 0.197 | 50.75M | nan | — | — |
| 100,000 | 1.974 | 50.65M | 1.909 | 52.39M | nan | — | — |
| 1,000,000 | 19.609 | 51.00M | 19.342 | 51.70M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.917 ms**; native kernel **1.905 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.213 | 0.150 | 6.64M | nan | — | — |
| 100,000 | 10 | 0.722 | 0.580 | 17.24M | nan | — | — |
| 100,000 | 1,000 | 20.785 | 19.533 | 51.19M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 46.69M | 47.86M | 1.00× | 2.78M | 3.15M | 1.00× | — |
| 2 | 44.93M | 47.22M | 0.99× | 3.28M | 3.20M | 1.02× | — |
| 4 | 45.26M | 45.44M | 0.95× | 2.97M | 3.43M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
