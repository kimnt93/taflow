# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.73M | 0.039 | 25.91M | 0.037 | 0.69× | 0.95× |
| 10,000 | 0.415 | 24.12M | 0.423 | 23.66M | 0.113 | 0.27× | 0.27× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.104 ms**; native kernel **0.074 ms**; TA-Lib 0.055 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.780 | 0.376 | 2.66M | 54.123 | 144.05× | 120.59× |
| 1,500 | 10 | 3.942 | 1.966 | 5.09M | 50.627 | 25.76× | 20.25× |
| 1,500 | 100 | 12.153 | 9.937 | 10.06M | 45.224 | 4.55× | 4.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
