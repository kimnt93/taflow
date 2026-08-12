# FourPointHarmonicPattern benchmark (`Abcd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.61M | 0.017 | 58.81M | 0.243 | 15.93× | 14.28× |
| 10,000 | 0.103 | 96.94M | 0.099 | 101.44M | 1.445 | 14.01× | 14.66× |
| 100,000 | 1.233 | 81.10M | 1.066 | 93.78M | 13.427 | 10.89× | 12.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.223 | 3.21× |
| 1 | 5 | 0.304 | 0.846 | 2.78× |
| 1 | 10 | 0.508 | 1.691 | 3.33× |
| 10 | 1 | 0.062 | 0.190 | 3.07× |
| 10 | 5 | 0.259 | 1.120 | 4.33× |
| 10 | 10 | 0.552 | 1.721 | 3.12× |
| 100 | 1 | 0.058 | 0.180 | 3.13× |
| 100 | 5 | 0.278 | 1.195 | 4.30× |
| 100 | 10 | 0.539 | 1.914 | 3.55× |
| 1,000 | 1 | 0.066 | 0.316 | 4.80× |
| 1,000 | 5 | 0.263 | 1.763 | 6.70× |
| 1,000 | 10 | 0.576 | 3.075 | 5.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
