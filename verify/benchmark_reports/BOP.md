# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 221.74M | 0.003 | 397.86M | 0.028 | 6.20× | 11.12× |
| 10,000 | 0.017 | 595.20M | 0.013 | 797.81M | 0.037 | 2.23× | 2.98× |
| 100,000 | 0.132 | 759.22M | 0.108 | 927.31M | 0.126 | 0.96× | 1.17× |
| 1,000,000 | 2.217 | 451.00M | 1.795 | 557.25M | 1.756 | 0.79× | 0.98× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.134 ms**; native kernel **0.107 ms**; TA-Lib 0.126 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.318 | 0.259 | 3.86M | 125.928 | 486.39× | 99.86× |
| 100,000 | 10 | 2.315 | 1.038 | 9.64M | 132.137 | 127.33× | 25.47× |
| 100,000 | 1,000 | 5.427 | 3.205 | 311.98M | 126.215 | 39.38× | 8.87× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 371.13M | 467.26M | 1.00× | 2.63M | 2.59M | 1.00× | 428.90M |
| 2 | 607.03M | 949.37M | 2.03× | 2.50M | 2.73M | 1.06× | 437.03M |
| 4 | 803.92M | 1.40G | 3.01× | 2.44M | 2.51M | 0.97× | 457.69M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
