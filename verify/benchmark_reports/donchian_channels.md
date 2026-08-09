# Donchian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.53M | 0.006 | 167.14M | nan | — | — |
| 10,000 | 0.080 | 124.42M | 0.074 | 135.10M | nan | — | — |
| 100,000 | 0.822 | 121.68M | 0.739 | 135.36M | nan | — | — |
| 1,000,000 | 20.579 | 48.59M | 8.993 | 111.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.805 ms**; native kernel **0.739 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.390 | 0.354 | 2.83M | nan | — | — |
| 100,000 | 10 | 2.356 | 1.695 | 5.90M | nan | — | — |
| 100,000 | 1,000 | 98.392 | 103.552 | 9.66M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 83.87M | 104.57M | 1.00× | 1.64M | 1.57M | 1.00× | — |
| 2 | 130.89M | 192.90M | 1.84× | 1.78M | 1.80M | 1.14× | — |
| 4 | 145.84M | 249.34M | 2.38× | 1.46M | 1.44M | 0.92× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
