# McGinleyDynamic benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.87M | 0.013 | 77.37M | nan | — | — |
| 10,000 | 0.125 | 80.10M | 0.121 | 82.49M | nan | — | — |
| 100,000 | 1.223 | 81.77M | 1.195 | 83.67M | nan | — | — |
| 1,000,000 | 12.478 | 80.14M | 12.055 | 82.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.234 ms**; native kernel **1.213 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.188 | 0.138 | 7.25M | nan | — | — |
| 100,000 | 10 | 0.934 | 0.570 | 17.54M | nan | — | — |
| 100,000 | 1,000 | 14.025 | 13.425 | 74.49M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 66.49M | 69.63M | 1.00× | 3.40M | 3.40M | 1.00× | — |
| 2 | 127.72M | 111.14M | 1.60× | 3.27M | 3.67M | 1.08× | — |
| 4 | 230.19M | 262.91M | 3.78× | 3.59M | 3.73M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
