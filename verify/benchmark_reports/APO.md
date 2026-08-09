# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.09M | 0.006 | 176.38M | 0.039 | 5.78× | 6.84× |
| 10,000 | 0.047 | 211.34M | 0.043 | 231.46M | 0.077 | 1.62× | 1.77× |
| 100,000 | 0.440 | 227.41M | 0.416 | 240.43M | 0.424 | 0.97× | 1.02× |
| 1,000,000 | 4.551 | 219.74M | 4.217 | 237.14M | 4.560 | 1.00× | 1.08× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.445 ms**; native kernel **0.429 ms**; TA-Lib 0.428 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.151 | 6.62M | 432.086 | 2861.14× | 222.03× |
| 100,000 | 10 | 0.912 | 0.536 | 18.67M | 425.620 | 794.77× | 63.47× |
| 100,000 | 1,000 | 6.906 | 5.822 | 171.76M | 440.928 | 75.73× | 6.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 169.75M | 174.50M | 1.00× | 2.96M | 3.42M | 1.00× | 170.35M |
| 2 | 311.43M | 348.82M | 2.00× | 2.95M | 4.39M | 1.28× | 178.71M |
| 4 | 504.31M | 634.83M | 3.64× | 2.91M | 3.12M | 0.91× | 168.57M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
