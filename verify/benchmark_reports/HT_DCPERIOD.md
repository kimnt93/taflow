# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.62M | 0.047 | 21.38M | 0.080 | 1.57× | 1.71× |
| 10,000 | 0.465 | 21.51M | 0.463 | 21.62M | 0.476 | 1.02× | 1.03× |
| 100,000 | 4.534 | 22.06M | 4.575 | 21.86M | 4.598 | 1.01× | 1.01× |
| 1,000,000 | 46.775 | 21.38M | 44.710 | 22.37M | 45.779 | 0.98× | 1.02× |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.732 ms**; native kernel **4.517 ms**; TA-Lib 4.391 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.304 | 0.212 | 4.71M | 4734.059 | 22297.37× | 134.74× |
| 100,000 | 10 | 1.500 | 1.100 | 9.09M | 4403.509 | 4004.30× | 25.50× |
| 100,000 | 1,000 | 57.003 | 45.008 | 22.22M | 4436.627 | 98.57× | 1.66× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22.25M | 21.08M | 1.00× | 2.87M | 3.18M | 1.00× | 20.56M |
| 2 | 38.59M | 40.59M | 1.93× | 2.62M | 2.81M | 0.88× | 20.60M |
| 4 | 70.99M | 77.49M | 3.68× | 2.30M | 2.52M | 0.79× | 20.75M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
