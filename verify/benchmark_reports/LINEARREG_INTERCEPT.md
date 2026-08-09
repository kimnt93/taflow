# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.13M | 0.014 | 69.93M | 0.044 | 2.69× | 3.08× |
| 10,000 | 0.137 | 73.22M | 0.125 | 79.75M | 0.152 | 1.12× | 1.21× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.020 ms**; TA-Lib 0.049 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.312 | 0.230 | 4.34M | 50.246 | 218.20× | 133.61× |
| 1,500 | 10 | 1.337 | 0.803 | 12.45M | 48.342 | 60.18× | 37.60× |
| 1,500 | 100 | 4.198 | 3.017 | 33.15M | 49.792 | 16.50× | 10.52× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.20M | 14.48M | 1.00× | 834.97K | 1.48M | 1.00× | 8.98M |
| 2 | 16.78M | 21.62M | 1.49× | 1.42M | 1.42M | 0.96× | 9.02M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
