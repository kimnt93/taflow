# RollingArgmin benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.36M | 0.006 | 167.29M | 0.035 | 4.83× | 5.84× |
| 10,000 | 0.052 | 191.08M | 0.052 | 193.36M | 0.092 | 1.76× | 1.79× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.311 | 0.230 | 4.34M | 37.787 | 164.15× | 123.56× |
| 1,500 | 10 | 1.161 | 0.631 | 15.85M | 38.213 | 60.57× | 47.58× |
| 1,500 | 100 | 4.174 | 2.714 | 36.85M | 39.409 | 14.52× | 10.81× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.85M | 16.38M | 1.00× | 1.07M | 1.52M | 1.00× | 8.91M |
| 2 | 18.59M | 19.35M | 1.18× | 1.38M | 1.62M | 1.07× | 9.97M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
