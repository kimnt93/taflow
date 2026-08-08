# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.23M | 0.006 | 159.69M | 0.034 | 0.72× | 5.44× |
| 10,000 | 0.461 | 21.71M | 0.053 | 188.96M | 0.053 | 0.11× | 1.00× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.073 ms**; native kernel **0.009 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.330 | 0.184 | 5.45M | 34.855 | 189.85× | 165.63× |
| 1,500 | 10 | 1.693 | 0.688 | 14.55M | 33.680 | 48.99× | 45.74× |
| 1,500 | 100 | 8.436 | 2.570 | 38.91M | 36.458 | 14.19× | 12.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
