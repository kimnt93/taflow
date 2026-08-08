# SignedPower benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.46M | 0.018 | 55.53M | nan | — | — |
| 10,000 | 0.172 | 58.01M | 0.169 | 59.34M | nan | — | — |
| 100,000 | 1.657 | 60.34M | 1.688 | 59.24M | nan | — | — |
| 1,000,000 | 17.153 | 58.30M | 16.846 | 59.36M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.665 ms**; native kernel **1.637 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.184 | 5.43M | nan | — | — |
| 100,000 | 10 | 1.137 | 0.676 | 14.80M | nan | — | — |
| 100,000 | 1,000 | 21.941 | 27.311 | 36.62M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 48.90M | 55.39M | 1.00× | 3.17M | 2.73M | 1.00× | — |
| 2 | 99.67M | 110.43M | 1.99× | 3.20M | 3.33M | 1.22× | — |
| 4 | 153.96M | 199.54M | 3.60× | 2.65M | 2.95M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
