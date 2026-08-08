# RollingAutocorr benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.89M | 0.057 | 17.57M | nan | — | — |
| 10,000 | 0.540 | 18.52M | 0.548 | 18.24M | nan | — | — |
| 100,000 | 5.660 | 17.67M | 5.486 | 18.23M | nan | — | — |
| 1,000,000 | 55.157 | 18.13M | 53.890 | 18.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.340 ms**; native kernel **5.291 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.273 | 0.207 | 4.83M | nan | — | — |
| 100,000 | 10 | 1.514 | 1.013 | 9.87M | nan | — | — |
| 100,000 | 1,000 | 62.992 | 53.566 | 18.67M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 17.39M | 17.84M | 1.00× | 2.58M | 2.60M | 1.00× | — |
| 2 | 29.36M | 30.39M | 1.70× | 2.89M | 2.54M | 0.98× | — |
| 4 | 53.09M | 48.33M | 2.71× | 2.61M | 2.69M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
