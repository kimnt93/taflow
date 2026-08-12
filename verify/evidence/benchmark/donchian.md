# Donchian benchmark (`Donchian` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.10M | 0.009 | 110.00M | 0.629 | 55.43× | 69.21× |
| 10,000 | 0.088 | 113.95M | 0.078 | 128.98M | 4.297 | 48.96× | 55.42× |
| 100,000 | 0.867 | 115.32M | 0.762 | 131.21M | 47.415 | 54.68× | 62.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.286 | 3.52× |
| 1 | 5 | 0.298 | 1.180 | 3.96× |
| 1 | 10 | 0.509 | 2.453 | 4.82× |
| 10 | 1 | 0.054 | 0.229 | 4.27× |
| 10 | 5 | 0.255 | 1.455 | 5.71× |
| 10 | 10 | 0.467 | 2.557 | 5.47× |
| 100 | 1 | 0.055 | 0.274 | 4.98× |
| 100 | 5 | 0.256 | 1.654 | 6.45× |
| 100 | 10 | 0.515 | 3.020 | 5.87× |
| 1,000 | 1 | 0.068 | 0.941 | 13.76× |
| 1,000 | 5 | 0.267 | 3.774 | 14.13× |
| 1,000 | 10 | 0.574 | 7.909 | 13.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
