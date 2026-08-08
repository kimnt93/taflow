# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.47M | 0.016 | 60.70M | 0.044 | 2.42× | 2.65× |
| 10,000 | 0.194 | 51.62M | 0.195 | 51.16M | 0.186 | 0.96× | 0.95× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.028 ms**; native kernel **0.025 ms**; TA-Lib 0.052 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.380 | 0.291 | 3.44M | 52.282 | 179.72× | 103.37× |
| 1,500 | 10 | 2.997 | 1.482 | 6.75M | 51.621 | 34.84× | 19.97× |
| 1,500 | 100 | 9.891 | 6.082 | 16.44M | 53.742 | 8.84× | 5.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
