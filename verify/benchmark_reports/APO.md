# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.03M | 0.007 | 133.64M | 0.039 | 4.88× | 5.26× |
| 10,000 | 0.048 | 207.90M | 0.045 | 222.43M | 0.075 | 1.57× | 1.68× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.241 | 0.171 | 5.86M | 41.191 | 241.49× | 222.92× |
| 1,500 | 10 | 2.052 | 1.259 | 7.95M | 41.377 | 32.87× | 27.43× |
| 1,500 | 100 | 3.037 | 2.173 | 46.01M | 41.910 | 19.28× | 16.56× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.80M | 16.05M | 1.00× | 1.33M | 1.54M | 1.00× | 7.76M |
| 2 | 17.63M | 20.41M | 1.27× | 1.56M | 1.49M | 0.96× | 8.98M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
