# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 280.74M | 0.002 | 583.18M | 0.031 | 8.62× | 17.90× |
| 10,000 | 0.014 | 738.12M | 0.009 | 1.07G | 0.036 | 2.67× | 3.86× |
| 100,000 | 0.097 | 1.03G | 0.071 | 1.41G | 0.094 | 0.97× | 1.33× |
| 1,000,000 | 2.165 | 461.92M | 1.372 | 728.63M | 1.412 | 0.65× | 1.03× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.106 ms**; native kernel **0.074 ms**; TA-Lib 0.098 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.371 | 0.280 | 3.57M | 105.803 | 378.12× | 103.84× |
| 100,000 | 10 | 2.079 | 1.118 | 8.95M | 98.718 | 88.33× | 25.45× |
| 100,000 | 1,000 | 4.490 | 5.474 | 182.69M | 94.110 | 17.19× | 5.00× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 351.13M | 610.04M | 1.00× | 2.24M | 2.80M | 1.00× | 419.69M |
| 2 | 678.53M | 975.53M | 1.60× | 2.32M | 3.07M | 1.10× | 507.71M |
| 4 | 678.65M | 1.44G | 2.35× | 2.34M | 2.69M | 0.96× | 511.01M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
