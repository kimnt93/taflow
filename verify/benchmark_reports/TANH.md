# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.68M | 0.005 | 182.29M | 0.035 | 5.52× | 6.46× |
| 10,000 | 0.039 | 259.32M | 0.037 | 273.13M | 0.055 | 1.43× | 1.50× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.284 | 0.162 | 6.18M | 30.015 | 185.60× | 153.64× |
| 1,500 | 10 | 1.101 | 0.598 | 16.72M | 29.946 | 50.07× | 41.46× |
| 1,500 | 100 | 3.133 | 2.030 | 49.26M | 30.326 | 14.94× | 12.58× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.42M | 14.42M | 1.00× | 1.18M | 1.24M | 1.00× | 10.12M |
| 2 | 13.54M | 19.58M | 1.36× | 1.31M | 1.78M | 1.43× | 10.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
