# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.330 | 3.03M | 0.022 | 45.98M | 0.042 | 0.13× | 1.93× |
| 10,000 | 3.322 | 3.01M | 0.248 | 40.38M | 0.119 | 0.04× | 0.48× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.518 ms**; native kernel **0.035 ms**; TA-Lib 0.048 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.390 | 0.240 | 4.17M | 47.530 | 198.42× | 152.41× |
| 1,500 | 10 | 4.687 | 2.940 | 3.40M | 46.729 | 15.89× | 12.28× |
| 1,500 | 100 | 32.091 | 5.011 | 19.96M | 48.601 | 9.70× | 7.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
