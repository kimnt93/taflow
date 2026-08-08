# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.11M | 0.009 | 107.75M | 0.039 | 3.77× | 4.19× |
| 10,000 | 0.087 | 114.79M | 0.080 | 124.30M | 0.076 | 0.88× | 0.95× |
| 100,000 | 0.792 | 126.24M | 0.784 | 127.57M | 0.437 | 0.55× | 0.56× |
| 1,000,000 | 8.093 | 123.56M | 7.546 | 132.51M | 4.704 | 0.58× | 0.62× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.801 ms**; native kernel **0.770 ms**; TA-Lib 0.453 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.272 | 0.175 | 5.71M | 436.623 | 2492.04× | 199.77× |
| 100,000 | 10 | 1.215 | 0.614 | 16.30M | 441.800 | 720.10× | 57.09× |
| 100,000 | 1,000 | 10.880 | 10.955 | 91.28M | 503.997 | 46.01× | 3.72× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 105.13M | 117.59M | 1.00× | 3.34M | 4.02M | 1.00× | 175.77M |
| 2 | 217.95M | 224.72M | 1.91× | 3.31M | 3.15M | 0.78× | 180.02M |
| 4 | 346.31M | 431.07M | 3.67× | 3.02M | 2.93M | 0.73× | 164.68M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
