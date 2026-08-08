# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.70M | 0.001 | 673.87M | 0.030 | 0.68× | 20.07× |
| 10,000 | 0.438 | 22.82M | 0.007 | 1.53G | 0.034 | 0.08× | 5.25× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.070 ms**; native kernel **0.002 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.371 | 0.253 | 3.95M | 32.013 | 126.54× | 112.07× |
| 1,500 | 10 | 2.648 | 1.033 | 9.68M | 28.801 | 27.87× | 25.88× |
| 1,500 | 100 | 8.742 | 2.644 | 37.82M | 30.070 | 11.37× | 10.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
