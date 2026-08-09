# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.55M | 0.005 | 187.83M | 0.034 | 5.76× | 6.38× |
| 10,000 | 0.045 | 220.63M | 0.043 | 231.30M | 0.052 | 1.15× | 1.20× |
| 100,000 | 0.442 | 226.24M | 0.416 | 240.42M | 0.212 | 0.48× | 0.51× |
| 1,000,000 | 4.622 | 216.35M | 4.130 | 242.12M | 1.894 | 0.41× | 0.46× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.437 ms**; native kernel **0.411 ms**; TA-Lib 0.217 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.211 | 0.153 | 6.52M | 215.258 | 1402.76× | 208.69× |
| 100,000 | 10 | 0.556 | 0.507 | 19.74M | 213.812 | 422.01× | 61.88× |
| 100,000 | 1,000 | 5.655 | 5.378 | 185.95M | 214.825 | 39.95× | 6.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 175.24M | 192.54M | 1.00× | 3.07M | 3.41M | 1.00× | 312.86M |
| 2 | 343.23M | 362.84M | 1.88× | 3.32M | 3.57M | 1.05× | 327.70M |
| 4 | 518.84M | 695.57M | 3.61× | 3.03M | 3.34M | 0.98× | 313.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
