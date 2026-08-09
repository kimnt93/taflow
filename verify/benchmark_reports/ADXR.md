# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.80M | 0.013 | 76.93M | 0.042 | 3.04× | 3.21× |
| 10,000 | 0.105 | 94.98M | 0.108 | 92.64M | 0.120 | 1.14× | 1.11× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.019 ms**; native kernel **0.019 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.447 | 0.306 | 3.26M | 46.470 | 151.69× | 118.55× |
| 1,500 | 10 | 1.273 | 2.077 | 4.81M | 48.871 | 23.53× | 15.16× |
| 1,500 | 100 | 3.759 | 3.253 | 30.74M | 51.944 | 15.97× | 10.71× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.99M | 11.51M | 1.00× | 874.65K | 1.39M | 1.00× | 7.76M |
| 2 | 16.18M | 16.42M | 1.43× | 1.24M | 1.43M | 1.04× | 9.57M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
