# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.06M | 0.016 | 62.83M | 0.035 | 1.77× | 2.18× |
| 10,000 | 0.162 | 61.88M | 0.158 | 63.28M | 0.130 | 0.80× | 0.82× |
| 100,000 | 1.643 | 60.87M | 1.585 | 63.09M | 1.029 | 0.63× | 0.65× |
| 1,000,000 | 16.129 | 62.00M | 15.929 | 62.78M | 10.789 | 0.67× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.124 | 1.05× |
| 1 | 5 | 0.254 | 0.430 | 1.69× |
| 1 | 10 | 0.612 | 1.016 | 1.66× |
| 10 | 1 | 0.063 | 0.090 | 1.41× |
| 10 | 5 | 0.252 | 0.444 | 1.76× |
| 10 | 10 | 0.555 | 1.202 | 2.17× |
| 100 | 1 | 0.069 | 0.098 | 1.42× |
| 100 | 5 | 0.265 | 0.437 | 1.65× |
| 100 | 10 | 0.566 | 1.010 | 1.78× |
| 1,000 | 1 | 0.089 | 0.132 | 1.48× |
| 1,000 | 5 | 0.291 | 0.559 | 1.92× |
| 1,000 | 10 | 0.596 | 1.017 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
