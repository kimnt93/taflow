# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.83M | 0.011 | 88.26M | 0.041 | 3.62× | 3.64× |
| 10,000 | 0.111 | 89.80M | 0.113 | 88.11M | 0.095 | 0.85× | 0.83× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.016 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.320 | 0.206 | 4.86M | 40.730 | 197.97× | 164.77× |
| 1,500 | 10 | 0.982 | 0.893 | 11.20M | 43.023 | 48.20× | 39.12× |
| 1,500 | 100 | 3.813 | 2.986 | 33.49M | 41.351 | 13.85× | 11.95× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.36M | 16.15M | 1.00× | 885.69K | 1.17M | 1.00× | 7.84M |
| 2 | 17.00M | 21.56M | 1.33× | 1.23M | 1.56M | 1.34× | 9.22M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
