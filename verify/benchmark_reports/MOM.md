# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 192.13M | 0.004 | 242.06M | 0.031 | 6.00× | 7.56× |
| 10,000 | 0.022 | 452.60M | 0.019 | 514.95M | 0.035 | 1.57× | 1.79× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.286 | 0.161 | 6.19M | 30.603 | 189.57× | 180.72× |
| 1,500 | 10 | 1.057 | 0.553 | 18.08M | 29.864 | 54.01× | 57.32× |
| 1,500 | 100 | 3.058 | 1.905 | 52.50M | 30.562 | 16.05× | 15.52× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.87M | 8.57M | 1.00× | 903.39K | 1.49M | 1.00× | 9.72M |
| 2 | 12.15M | 19.40M | 2.26× | 1.51M | 1.87M | 1.26× | 9.78M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
