# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.66M | 0.007 | 142.02M | 0.031 | 3.82× | 4.35× |
| 10,000 | 0.052 | 193.08M | 0.051 | 196.07M | 0.068 | 1.31× | 1.33× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.308 | 0.214 | 4.68M | 33.121 | 154.97× | 117.50× |
| 1,500 | 10 | 1.139 | 0.610 | 16.39M | 33.663 | 55.18× | 42.39× |
| 1,500 | 100 | 3.454 | 2.121 | 47.15M | 33.798 | 15.93× | 12.70× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.42M | 17.34M | 1.00× | 1.33M | 1.62M | 1.00× | 9.95M |
| 2 | 12.51M | 21.11M | 1.22× | 1.14M | 1.55M | 0.96× | 10.45M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
