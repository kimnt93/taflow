# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.94M | 0.014 | 70.58M | 0.050 | 3.16× | 3.55× |
| 10,000 | 0.135 | 73.82M | 0.134 | 74.83M | 0.179 | 1.32× | 1.34× |
| 100,000 | 1.278 | 78.22M | 1.274 | 78.50M | 1.461 | 1.14× | 1.15× |
| 1,000,000 | 13.471 | 74.23M | 13.116 | 76.24M | 14.396 | 1.07× | 1.10× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.304 ms**; native kernel **1.268 ms**; TA-Lib 1.465 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.300 | 0.244 | 4.10M | 1508.909 | 6182.25× | 139.58× |
| 100,000 | 10 | 1.989 | 1.102 | 9.07M | 1443.348 | 1309.27× | 30.30× |
| 100,000 | 1,000 | 16.814 | 14.820 | 67.48M | 1496.056 | 100.95× | 3.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 65.46M | 65.30M | 1.00× | 2.41M | 2.17M | 1.00× | 60.87M |
| 2 | 125.81M | 137.58M | 2.11× | 2.39M | 2.47M | 1.14× | 59.97M |
| 4 | 217.57M | 245.81M | 3.76× | 2.27M | 2.25M | 1.04× | 59.79M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
