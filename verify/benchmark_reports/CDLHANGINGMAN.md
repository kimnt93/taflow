# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.79M | 0.008 | 120.35M | 0.039 | 3.83× | 4.72× |
| 10,000 | 0.124 | 80.74M | 0.120 | 83.57M | 0.175 | 1.41× | 1.46× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.014 ms**; TA-Lib 0.049 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.362 | 0.445 | 2.25M | 48.471 | 108.83× | 64.68× |
| 1,500 | 10 | 2.600 | 1.303 | 7.67M | 52.082 | 39.97× | 22.61× |
| 1,500 | 100 | 10.171 | 5.985 | 16.71M | 51.825 | 8.66× | 5.05× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.01M | 9.82M | 1.00× | 1.19M | 1.18M | 1.00× | 6.91M |
| 2 | 17.08M | 19.35M | 1.97× | 1.32M | 1.36M | 1.15× | 8.82M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
