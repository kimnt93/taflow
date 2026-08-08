# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.00M | 0.011 | 87.40M | 0.038 | 2.75× | 3.29× |
| 10,000 | 0.151 | 66.44M | 0.146 | 68.57M | 0.179 | 1.19× | 1.23× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.017 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.380 | 0.291 | 3.43M | 40.261 | 138.13× | 97.24× |
| 1,500 | 10 | 2.731 | 1.266 | 7.90M | 37.978 | 30.00× | 23.06× |
| 1,500 | 100 | 7.015 | 4.108 | 24.34M | 39.544 | 9.63× | 7.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
