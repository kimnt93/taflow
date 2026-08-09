# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.31M | 0.005 | 191.08M | 0.033 | 5.42× | 6.23× |
| 10,000 | 0.044 | 227.06M | 0.041 | 241.21M | 0.060 | 1.36× | 1.45× |
| 100,000 | 0.413 | 241.92M | 0.392 | 255.39M | 0.314 | 0.76× | 0.80× |
| 1,000,000 | 4.428 | 225.81M | 4.088 | 244.61M | 2.874 | 0.65× | 0.70× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.417 ms**; native kernel **0.398 ms**; TA-Lib 0.314 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.158 | 6.35M | 309.257 | 1962.58× | 189.43× |
| 100,000 | 10 | 0.898 | 0.547 | 18.27M | 314.995 | 575.50× | 55.12× |
| 100,000 | 1,000 | 6.561 | 8.319 | 120.21M | 315.155 | 37.89× | 4.05× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 171.83M | 192.78M | 1.00× | 2.86M | 2.90M | 1.00× | 204.60M |
| 2 | 343.52M | 392.45M | 2.04× | 3.25M | 3.96M | 1.37× | 221.64M |
| 4 | 510.65M | 704.46M | 3.65× | 3.13M | 3.40M | 1.17× | 228.86M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
