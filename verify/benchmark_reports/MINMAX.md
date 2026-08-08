# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.89M | 0.008 | 132.62M | 0.044 | 5.01× | 5.84× |
| 10,000 | 0.086 | 116.43M | 0.079 | 127.11M | 0.121 | 1.41× | 1.54× |
| 100,000 | 0.856 | 116.86M | 0.817 | 122.41M | 0.891 | 1.04× | 1.09× |
| 1,000,000 | 17.205 | 58.12M | 15.305 | 65.34M | 8.433 | 0.49× | 0.55× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.896 ms**; native kernel **0.801 ms**; TA-Lib 0.875 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.303 | 0.247 | 4.05M | 868.792 | 3520.14× | 144.76× |
| 100,000 | 10 | 1.908 | 1.414 | 7.07M | 907.794 | 642.19× | 25.39× |
| 100,000 | 1,000 | 62.290 | 56.912 | 17.57M | 895.252 | 15.73× | 0.79× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 87.17M | 100.42M | 1.00× | 2.04M | 2.38M | 1.00× | 101.31M |
| 2 | 132.40M | 168.49M | 1.68× | 2.20M | 2.03M | 0.86× | 100.45M |
| 4 | 168.78M | 213.14M | 2.12× | 1.74M | 1.81M | 0.76× | 97.72M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
