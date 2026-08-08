# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.95M | 0.001 | 685.04M | 0.029 | 0.68× | 20.19× |
| 10,000 | 0.427 | 23.43M | 0.006 | 1.73G | 0.035 | 0.08× | 6.01× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.066 ms**; native kernel **0.002 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.330 | 0.237 | 4.21M | 28.665 | 120.71× | 119.95× |
| 1,500 | 10 | 2.599 | 1.030 | 9.71M | 30.513 | 29.62× | 28.15× |
| 1,500 | 100 | 8.289 | 2.515 | 39.76M | 29.480 | 11.72× | 10.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
