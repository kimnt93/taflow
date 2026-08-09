# RollingArgmax benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.46M | 0.007 | 147.12M | 0.035 | 4.34× | 5.13× |
| 10,000 | 0.055 | 183.31M | 0.051 | 197.82M | 0.092 | 1.69× | 1.83× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.307 | 0.179 | 5.58M | 38.914 | 217.17× | 163.99× |
| 1,500 | 10 | 1.239 | 0.680 | 14.70M | 37.216 | 54.72× | 41.41× |
| 1,500 | 100 | 4.062 | 2.634 | 37.97M | 38.845 | 14.75× | 13.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.76M | 12.67M | 1.00× | 1.08M | 1.13M | 1.00× | 9.11M |
| 2 | 14.56M | 19.94M | 1.57× | 1.37M | 1.70M | 1.50× | 9.65M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
