# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.73M | 0.035 | 28.44M | 0.039 | 1.00× | 1.10× |
| 10,000 | 0.375 | 26.67M | 0.372 | 26.91M | 0.127 | 0.34× | 0.34× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.059 ms**; native kernel **0.053 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.410 | 0.319 | 3.14M | 41.097 | 128.95× | 104.59× |
| 1,500 | 10 | 2.944 | 1.513 | 6.61M | 47.964 | 31.71× | 22.17× |
| 1,500 | 100 | 9.756 | 6.639 | 15.06M | 44.308 | 6.67× | 4.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
