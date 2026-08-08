# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 161.95M | 0.005 | 192.50M | 0.040 | 6.41× | 7.62× |
| 10,000 | 0.041 | 244.97M | 0.038 | 260.65M | 0.080 | 1.97× | 2.09× |
| 100,000 | 0.385 | 260.07M | 0.356 | 281.22M | 0.486 | 1.26× | 1.37× |
| 1,000,000 | 4.188 | 238.80M | 3.632 | 275.35M | 5.084 | 1.21× | 1.40× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.381 ms**; native kernel **0.357 ms**; TA-Lib 0.487 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.162 | 6.18M | 486.417 | 3008.46× | 203.53× |
| 100,000 | 10 | 1.027 | 0.561 | 17.83M | 500.349 | 892.14× | 61.58× |
| 100,000 | 1,000 | 9.078 | 5.118 | 195.38M | 492.185 | 96.16× | 7.62× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 167.61M | 212.00M | 1.00× | 3.09M | 3.45M | 1.00× | 160.24M |
| 2 | 324.56M | 438.08M | 2.07× | 3.03M | 3.77M | 1.09× | 157.38M |
| 4 | 512.47M | 632.46M | 2.98× | 2.98M | 3.15M | 0.91× | 149.20M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
