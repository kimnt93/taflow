# FibonacciExtension benchmark (`FibExtension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.86M | 0.017 | 57.84M | 0.594 | 28.43× | 34.36× |
| 10,000 | 0.163 | 61.26M | 0.152 | 65.78M | 4.828 | 29.58× | 31.76× |
| 100,000 | 1.711 | 58.44M | 1.475 | 67.79M | 52.145 | 30.47× | 35.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.241 | 2.96× |
| 1 | 5 | 0.264 | 0.855 | 3.24× |
| 1 | 10 | 0.446 | 1.853 | 4.15× |
| 10 | 1 | 0.055 | 0.183 | 3.33× |
| 10 | 5 | 0.244 | 0.866 | 3.55× |
| 10 | 10 | 0.487 | 2.024 | 4.16× |
| 100 | 1 | 0.056 | 0.225 | 3.98× |
| 100 | 5 | 0.248 | 1.132 | 4.57× |
| 100 | 10 | 0.491 | 2.448 | 4.99× |
| 1,000 | 1 | 0.072 | 0.858 | 11.85× |
| 1,000 | 5 | 0.241 | 3.636 | 15.06× |
| 1,000 | 10 | 0.506 | 7.309 | 14.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
