# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.42M | 0.002 | 486.68M | 0.030 | 0.64× | 14.56× |
| 10,000 | 0.435 | 23.00M | 0.008 | 1.27G | 0.036 | 0.08× | 4.56× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.069 ms**; native kernel **0.002 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.373 | 0.271 | 3.69M | 31.349 | 115.71× | 102.95× |
| 1,500 | 10 | 9.412 | 1.803 | 5.55M | 31.935 | 17.71× | 15.48× |
| 1,500 | 100 | 8.692 | 2.757 | 36.27M | 30.409 | 11.03× | 9.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
