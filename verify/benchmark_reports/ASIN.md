# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.85M | 0.007 | 138.65M | 0.032 | 4.02× | 4.50× |
| 10,000 | 0.080 | 125.62M | 0.069 | 145.85M | 0.088 | 1.10× | 1.28× |
| 100,000 | 0.700 | 142.76M | 0.680 | 147.03M | 0.633 | 0.90× | 0.93× |
| 1,000,000 | 7.993 | 125.12M | 7.624 | 131.16M | 6.173 | 0.77× | 0.81× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.700 ms**; native kernel **0.676 ms**; TA-Lib 0.642 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.244 | 0.168 | 5.96M | 644.092 | 3840.09× | 150.09× |
| 100,000 | 10 | 0.939 | 0.572 | 17.49M | 641.051 | 1121.46× | 44.20× |
| 100,000 | 1,000 | 9.629 | 9.023 | 110.83M | 636.678 | 70.56× | 3.58× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 110.95M | 125.36M | 1.00× | 2.83M | 3.28M | 1.00× | 133.20M |
| 2 | 202.37M | 229.58M | 1.83× | 3.30M | 3.15M | 0.96× | 127.07M |
| 4 | 285.19M | 385.19M | 3.07× | 2.84M | 3.26M | 1.00× | 128.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
