# RollingKurtosis benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.64M | 0.020 | 49.98M | nan | — | — |
| 10,000 | 0.235 | 42.51M | 0.179 | 55.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.028 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.243 | 0.176 | 5.68M | nan | — | — |
| 1,500 | 10 | 1.095 | 0.671 | 14.89M | nan | — | — |
| 1,500 | 100 | 3.280 | 2.890 | 34.60M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.33M | 7.21M | 1.00× | 949.45K | 1.47M | 1.00× | — |
| 2 | 14.58M | 17.93M | 2.49× | 1.53M | 1.43M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
