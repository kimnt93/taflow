# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.03M | 0.005 | 212.20M | 0.038 | 6.68× | 8.10× |
| 10,000 | 0.035 | 286.53M | 0.036 | 277.24M | 0.075 | 2.16× | 2.09× |
| 100,000 | 0.328 | 305.23M | 0.303 | 329.58M | 0.432 | 1.32× | 1.42× |
| 1,000,000 | 3.554 | 281.37M | 3.133 | 319.23M | 4.696 | 1.32× | 1.50× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.330 ms**; native kernel **0.302 ms**; TA-Lib 0.434 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.222 | 0.159 | 6.27M | 439.902 | 2758.41× | 214.48× |
| 100,000 | 10 | 0.950 | 0.611 | 16.37M | 434.032 | 710.49× | 55.15× |
| 100,000 | 1,000 | 6.573 | 5.060 | 197.63M | 446.768 | 88.30× | 7.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 213.60M | 237.13M | 1.00× | 2.70M | 3.45M | 1.00× | 175.97M |
| 2 | 400.59M | 537.38M | 2.27× | 3.15M | 3.60M | 1.04× | 175.84M |
| 4 | 608.20M | 830.87M | 3.50× | 3.11M | 3.42M | 0.99× | 175.45M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
