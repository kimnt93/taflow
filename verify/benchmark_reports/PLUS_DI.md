# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.05M | 0.011 | 89.66M | 0.038 | 2.94× | 3.43× |
| 10,000 | 0.101 | 99.28M | 0.103 | 96.95M | 0.100 | 0.99× | 0.97× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.017 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.344 | 0.276 | 3.62M | 41.030 | 148.71× | 115.76× |
| 1,500 | 10 | 2.264 | 1.042 | 9.60M | 41.284 | 39.62× | 31.03× |
| 1,500 | 100 | 6.564 | 3.944 | 25.35M | 41.977 | 10.64× | 7.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
