# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.65M | 0.005 | 195.26M | 0.032 | 4.54× | 6.26× |
| 10,000 | 0.043 | 230.04M | 0.035 | 286.30M | 0.052 | 1.20× | 1.49× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.007 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.817 | 0.291 | 3.44M | 32.035 | 110.13× | 101.14× |
| 1,500 | 10 | 2.606 | 1.143 | 8.75M | 32.301 | 28.27× | 24.49× |
| 1,500 | 100 | 6.018 | 2.949 | 33.91M | 33.207 | 11.26× | 9.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
