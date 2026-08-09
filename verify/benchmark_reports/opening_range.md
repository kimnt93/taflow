# OpeningRange benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.71M | 0.008 | 125.73M | nan | — | — |
| 10,000 | 0.066 | 150.43M | 0.059 | 170.33M | nan | — | — |
| 100,000 | 0.655 | 152.64M | 0.566 | 176.60M | nan | — | — |
| 1,000,000 | 7.643 | 130.84M | 6.384 | 156.65M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.624 ms**; native kernel **0.570 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.400 | 0.411 | 2.43M | nan | — | — |
| 100,000 | 10 | 1.705 | 1.039 | 9.62M | nan | — | — |
| 100,000 | 1,000 | 8.984 | 8.193 | 122.05M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 108.71M | 122.05M | 1.00× | 1.88M | 1.69M | 1.00× | — |
| 2 | 110.17M | 127.13M | 1.04× | 1.88M | 1.77M | 1.04× | — |
| 4 | 104.47M | 119.94M | 0.98× | 1.85M | 1.63M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
