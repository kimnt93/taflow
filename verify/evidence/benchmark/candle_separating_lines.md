# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.63M | 0.018 | 55.08M | 0.045 | 2.13× | 2.47× |
| 10,000 | 0.129 | 77.29M | 0.160 | 62.38M | 0.129 | 0.99× | 0.80× |
| 100,000 | 1.410 | 70.93M | 1.404 | 71.21M | 0.980 | 0.69× | 0.70× |
| 1,000,000 | 14.010 | 71.38M | 15.284 | 65.43M | 11.228 | 0.80× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.163 | 0.97× |
| 1 | 5 | 0.428 | 1.571 | 3.67× |
| 1 | 10 | 2.420 | 3.557 | 1.47× |
| 10 | 1 | 0.096 | 0.165 | 1.72× |
| 10 | 5 | 0.416 | 0.650 | 1.56× |
| 10 | 10 | 0.769 | 1.172 | 1.52× |
| 100 | 1 | 0.059 | 0.090 | 1.52× |
| 100 | 5 | 0.301 | 0.670 | 2.22× |
| 100 | 10 | 0.617 | 1.132 | 1.84× |
| 1,000 | 1 | 0.081 | 0.113 | 1.39× |
| 1,000 | 5 | 0.387 | 0.638 | 1.65× |
| 1,000 | 10 | 0.600 | 1.120 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
