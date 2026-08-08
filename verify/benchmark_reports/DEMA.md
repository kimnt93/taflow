# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.57M | 0.009 | 116.57M | 0.036 | 0.75× | 4.25× |
| 10,000 | 0.469 | 21.32M | 0.077 | 130.68M | 0.091 | 0.19× | 1.19× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.075 ms**; native kernel **0.012 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.290 | 0.185 | 5.42M | 41.267 | 223.59× | 164.70× |
| 1,500 | 10 | 1.615 | 0.723 | 13.84M | 40.070 | 55.45× | 42.14× |
| 1,500 | 100 | 7.106 | 2.826 | 35.39M | 41.851 | 14.81× | 14.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
