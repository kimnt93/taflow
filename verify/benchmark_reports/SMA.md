# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.70M | 0.005 | 191.01M | 0.033 | 0.71× | 6.24× |
| 10,000 | 0.448 | 22.33M | 0.044 | 226.68M | 0.051 | 0.11× | 1.15× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.068 ms**; native kernel **0.008 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.289 | 0.186 | 5.38M | 33.871 | 182.37× | 161.43× |
| 1,500 | 10 | 1.629 | 0.676 | 14.80M | 34.133 | 50.52× | 46.82× |
| 1,500 | 100 | 6.773 | 2.451 | 40.80M | 36.412 | 14.86× | 12.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
