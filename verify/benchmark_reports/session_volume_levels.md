# SessionVolumeLevels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.24M | 0.052 | 19.14M | nan | — | — |
| 10,000 | 0.475 | 21.07M | 0.463 | 21.62M | nan | — | — |
| 100,000 | 4.864 | 20.56M | 4.826 | 20.72M | nan | — | — |
| 1,000,000 | 64.528 | 15.50M | 49.027 | 20.40M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.899 ms**; native kernel **4.840 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.505 | 0.446 | 2.24M | nan | — | — |
| 100,000 | 10 | 2.468 | 1.780 | 5.62M | nan | — | — |
| 100,000 | 1,000 | 59.617 | 58.112 | 17.21M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18.77M | 19.61M | 1.00× | 1.28M | 1.55M | 1.00× | — |
| 2 | 18.44M | 19.49M | 0.99× | 1.49M | 1.47M | 0.95× | — |
| 4 | 18.10M | 18.41M | 0.94× | 1.35M | 1.46M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
