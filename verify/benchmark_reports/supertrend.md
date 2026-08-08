# Supertrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.70M | 0.018 | 54.51M | nan | — | — |
| 10,000 | 0.177 | 56.62M | 0.186 | 53.63M | nan | — | — |
| 100,000 | 1.672 | 59.82M | 1.758 | 56.88M | nan | — | — |
| 1,000,000 | 34.466 | 29.01M | 27.795 | 35.98M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.750 ms**; native kernel **1.771 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.358 | 0.296 | 3.38M | nan | — | — |
| 100,000 | 10 | 2.157 | 1.141 | 8.77M | nan | — | — |
| 100,000 | 1,000 | 28.841 | 23.442 | 42.66M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 47.90M | 55.49M | 1.00× | 1.86M | 2.04M | 1.00× | — |
| 2 | 81.42M | 100.05M | 1.80× | 2.21M | 2.10M | 1.03× | — |
| 4 | 109.57M | 149.80M | 2.70× | 2.21M | 2.18M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
