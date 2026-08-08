# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.09M | 0.020 | 49.26M | 0.051 | 2.31× | 2.52× |
| 10,000 | 0.180 | 55.46M | 0.187 | 53.54M | 0.172 | 0.96× | 0.92× |
| 100,000 | 1.688 | 59.25M | 1.755 | 56.97M | 1.254 | 0.74× | 0.71× |
| 1,000,000 | 19.216 | 52.04M | 22.828 | 43.81M | 13.349 | 0.69× | 0.58× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.742 ms**; native kernel **1.699 ms**; TA-Lib 1.283 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.428 | 0.397 | 2.52M | 1301.822 | 3278.83× | 111.46× |
| 100,000 | 10 | 2.347 | 2.306 | 4.34M | 1256.715 | 544.91× | 18.78× |
| 100,000 | 1,000 | 118.616 | 105.189 | 9.51M | 1266.700 | 12.04× | 0.48× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 43.35M | 57.96M | 1.00× | 1.46M | 1.69M | 1.00× | 69.12M |
| 2 | 86.25M | 104.13M | 1.80× | 1.69M | 1.67M | 0.99× | 66.96M |
| 4 | 145.90M | 197.22M | 3.40× | 1.69M | 1.74M | 1.03× | 63.18M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
