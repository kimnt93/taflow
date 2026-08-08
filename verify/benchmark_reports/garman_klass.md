# GarmanKlass benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.69M | 0.017 | 57.60M | nan | — | — |
| 10,000 | 0.157 | 63.54M | 0.151 | 66.24M | nan | — | — |
| 100,000 | 1.508 | 66.31M | 1.470 | 68.02M | nan | — | — |
| 1,000,000 | 15.630 | 63.98M | 15.620 | 64.02M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.553 ms**; native kernel **1.501 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.347 | 0.261 | 3.83M | nan | — | — |
| 100,000 | 10 | 2.585 | 1.229 | 8.14M | nan | — | — |
| 100,000 | 1,000 | 19.006 | 16.323 | 61.26M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 55.35M | 59.39M | 1.00× | 1.84M | 2.36M | 1.00× | — |
| 2 | 106.92M | 115.51M | 1.94× | 2.13M | 2.54M | 1.08× | — |
| 4 | 186.14M | 216.19M | 3.64× | 2.15M | 2.40M | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
