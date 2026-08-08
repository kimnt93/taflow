# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 44.38M | 0.020 | 50.49M | 0.053 | 2.36× | 2.68× |
| 10,000 | 0.194 | 51.43M | 0.202 | 49.58M | 0.243 | 1.25× | 1.21× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.031 ms**; TA-Lib 0.062 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.425 | 0.270 | 3.71M | 64.236 | 238.11× | 107.43× |
| 1,500 | 10 | 2.435 | 2.573 | 3.89M | 62.209 | 24.18× | 11.39× |
| 1,500 | 100 | 7.541 | 5.006 | 19.98M | 63.520 | 12.69× | 6.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
