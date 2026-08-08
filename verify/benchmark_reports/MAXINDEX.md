# RollingArgmax benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.26M | 0.011 | 91.83M | 0.036 | 0.70× | 3.34× |
| 10,000 | 0.556 | 18.00M | 0.150 | 66.51M | 0.104 | 0.19× | 0.69× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.078 ms**; native kernel **0.017 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.471 | 0.195 | 5.13M | 38.481 | 197.28× | 151.20× |
| 1,500 | 10 | 2.014 | 0.872 | 11.46M | 38.253 | 43.85× | 34.76× |
| 1,500 | 100 | 10.660 | 4.035 | 24.78M | 40.780 | 10.11× | 7.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
