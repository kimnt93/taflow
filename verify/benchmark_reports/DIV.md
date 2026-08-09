# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 380.10M | 0.001 | 745.62M | 0.028 | 10.80× | 21.19× |
| 10,000 | 0.009 | 1.11G | 0.006 | 1.71G | 0.034 | 3.77× | 5.79× |
| 100,000 | 0.070 | 1.43G | 0.047 | 2.12G | 0.075 | 1.07× | 1.59× |
| 1,000,000 | 1.150 | 869.89M | 0.788 | 1.27G | 0.826 | 0.72× | 1.05× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.071 ms**; native kernel **0.047 ms**; TA-Lib 0.075 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.182 | 5.48M | 76.907 | 421.78× | 151.00× |
| 100,000 | 10 | 1.409 | 0.738 | 13.54M | 75.324 | 102.02× | 38.85× |
| 100,000 | 1,000 | 3.938 | 2.219 | 450.60M | 75.474 | 34.01× | 12.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 493.09M | 916.17M | 1.00× | 2.91M | 3.31M | 1.00× | 669.00M |
| 2 | 966.60M | 1.22G | 1.33× | 2.57M | 3.60M | 1.09× | 555.53M |
| 4 | 850.39M | 1.97G | 2.15× | 2.90M | 3.12M | 0.94× | 629.15M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
