# RollingProfitFactor benchmark (`ProfitFactor` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.56M | 0.026 | 37.87M | 0.065 | 2.39× | 2.48× |
| 10,000 | 0.244 | 40.99M | 0.245 | 40.84M | 0.529 | 2.17× | 2.16× |
| 100,000 | 2.441 | 40.97M | 2.492 | 40.12M | 5.437 | 2.23× | 2.18× |
| 1,000,000 | 24.949 | 40.08M | 25.271 | 39.57M | 52.312 | 2.10× | 2.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.108 | 1.14× |
| 1 | 5 | 0.242 | 0.338 | 1.39× |
| 1 | 10 | 0.471 | 0.696 | 1.48× |
| 10 | 1 | 0.055 | 0.073 | 1.34× |
| 10 | 5 | 0.235 | 0.383 | 1.63× |
| 10 | 10 | 0.466 | 0.650 | 1.40× |
| 100 | 1 | 0.047 | 0.073 | 1.54× |
| 100 | 5 | 0.222 | 0.354 | 1.60× |
| 100 | 10 | 0.530 | 0.767 | 1.45× |
| 1,000 | 1 | 0.073 | 0.123 | 1.67× |
| 1,000 | 5 | 0.224 | 0.595 | 2.65× |
| 1,000 | 10 | 0.516 | 1.302 | 2.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
