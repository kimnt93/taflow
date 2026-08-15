# MovingAverageEnvelope benchmark (`MaEnvelope` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.47M | 0.022 | 45.71M | 0.524 | 22.76× | 23.94× |
| 10,000 | 0.215 | 46.62M | 0.197 | 50.86M | 3.666 | 17.09× | 18.65× |
| 100,000 | 1.953 | 51.21M | 1.939 | 51.57M | 40.648 | 20.82× | 20.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.307 | 4.62× |
| 1 | 5 | 0.255 | 1.395 | 5.46× |
| 1 | 10 | 0.401 | 2.722 | 6.79× |
| 10 | 1 | 0.043 | 0.260 | 6.01× |
| 10 | 5 | 0.190 | 1.459 | 7.69× |
| 10 | 10 | 0.420 | 2.847 | 6.78× |
| 100 | 1 | 0.050 | 0.281 | 5.58× |
| 100 | 5 | 0.228 | 1.651 | 7.25× |
| 100 | 10 | 0.496 | 3.020 | 6.09× |
| 1,000 | 1 | 0.064 | 0.816 | 12.67× |
| 1,000 | 5 | 0.214 | 3.369 | 15.77× |
| 1,000 | 10 | 0.471 | 14.247 | 30.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
