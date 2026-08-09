# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.157 | 6.37M | 0.163 | 6.12M | 0.112 | 0.71× | 0.69× |
| 10,000 | 1.514 | 6.61M | 1.253 | 7.98M | 0.839 | 0.55× | 0.67× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.244 ms**; native kernel **0.182 ms**; TA-Lib 0.154 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.459 | 0.335 | 2.99M | 154.336 | 460.90× | 119.97× |
| 1,500 | 10 | 2.773 | 2.061 | 4.85M | 150.097 | 72.84× | 19.62× |
| 1,500 | 100 | 17.203 | 13.193 | 7.58M | 163.700 | 12.41× | 3.67× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 4.22M | 4.03M | 1.00× | 968.33K | 848.94K | 1.00× | 4.26M |
| 2 | 8.31M | 9.29M | 2.30× | 978.94K | 882.05K | 1.04× | 4.45M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
