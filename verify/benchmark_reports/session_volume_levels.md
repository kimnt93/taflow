# SessionVolumeLevels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.41M | 0.053 | 18.84M | nan | — | — |
| 10,000 | 0.511 | 19.56M | 0.526 | 19.02M | nan | — | — |
| 100,000 | 5.467 | 18.29M | 5.315 | 18.82M | nan | — | — |
| 1,000,000 | 72.896 | 13.72M | 53.702 | 18.62M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.505 ms**; native kernel **5.080 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.529 | 0.513 | 1.95M | nan | — | — |
| 100,000 | 10 | 2.668 | 1.876 | 5.33M | nan | — | — |
| 100,000 | 1,000 | 70.537 | 59.969 | 16.68M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18.30M | 19.37M | 1.00× | 1.41M | 1.40M | 1.00× | — |
| 2 | 18.38M | 18.73M | 0.97× | 1.40M | 1.51M | 1.08× | — |
| 4 | 17.37M | 17.67M | 0.91× | 1.40M | 1.46M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
