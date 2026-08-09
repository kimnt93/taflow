# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.73M | 0.009 | 112.24M | 0.042 | 4.11× | 4.77× |
| 10,000 | 0.101 | 99.15M | 0.094 | 106.54M | 0.151 | 1.49× | 1.60× |
| 100,000 | 1.049 | 95.33M | 0.984 | 101.66M | 1.145 | 1.09× | 1.16× |
| 1,000,000 | 10.858 | 92.10M | 10.253 | 97.54M | 11.602 | 1.07× | 1.13× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.026 ms**; native kernel **0.959 ms**; TA-Lib 1.155 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.347 | 0.250 | 4.00M | 1131.434 | 4524.69× | 129.45× |
| 100,000 | 10 | 1.690 | 1.175 | 8.51M | 1141.183 | 971.16× | 27.93× |
| 100,000 | 1,000 | 87.001 | 90.890 | 11.00M | 1166.708 | 12.84× | 0.50× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 76.17M | 84.46M | 1.00× | 2.08M | 2.11M | 1.00× | 71.65M |
| 2 | 135.96M | 169.02M | 2.00× | 1.95M | 2.59M | 1.23× | 72.22M |
| 4 | 217.25M | 301.60M | 3.57× | 2.00M | 2.36M | 1.12× | 73.53M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
