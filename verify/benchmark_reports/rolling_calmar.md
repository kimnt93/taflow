# RollingCalmar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.47M | 0.019 | 51.92M | nan | — | — |
| 10,000 | 0.186 | 53.79M | 0.176 | 56.68M | nan | — | — |
| 100,000 | 1.797 | 55.64M | 1.805 | 55.39M | nan | — | — |
| 1,000,000 | 18.203 | 54.94M | 19.162 | 52.19M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.799 ms**; native kernel **1.766 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.217 | 0.158 | 6.32M | nan | — | — |
| 100,000 | 10 | 0.993 | 0.643 | 15.54M | nan | — | — |
| 100,000 | 1,000 | 20.508 | 20.822 | 48.03M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 49.63M | 49.15M | 1.00× | 2.61M | 3.65M | 1.00× | — |
| 2 | 89.54M | 97.91M | 1.99× | 3.24M | 3.06M | 0.84× | — |
| 4 | 91.16M | 99.19M | 2.02× | 3.03M | 3.14M | 0.86× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
