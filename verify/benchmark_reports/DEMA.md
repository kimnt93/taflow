# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.91M | 0.006 | 161.77M | 0.036 | 5.08× | 5.83× |
| 10,000 | 0.056 | 177.63M | 0.052 | 192.00M | 0.089 | 1.58× | 1.70× |
| 100,000 | 0.529 | 188.86M | 0.501 | 199.77M | 0.601 | 1.14× | 1.20× |
| 1,000,000 | 5.611 | 178.24M | 5.086 | 196.62M | 6.398 | 1.14× | 1.26× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.530 ms**; native kernel **0.509 ms**; TA-Lib 0.602 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.212 | 0.147 | 6.79M | 599.360 | 4066.71× | 200.16× |
| 100,000 | 10 | 0.875 | 0.580 | 17.25M | 606.753 | 1046.38× | 51.05× |
| 100,000 | 1,000 | 7.447 | 6.475 | 154.44M | 627.896 | 96.97× | 5.59× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 138.78M | 148.52M | 1.00× | 3.00M | 3.15M | 1.00× | 136.83M |
| 2 | 266.13M | 305.48M | 2.06× | 3.45M | 4.08M | 1.29× | 126.72M |
| 4 | 406.65M | 555.22M | 3.74× | 2.96M | 3.20M | 1.02× | 123.98M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
