# Lag benchmark (`causal lag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 518.67M | 0.001 | 1.03G | 0.026 | 13.55× | 26.91× |
| 10,000 | 0.006 | 1.54G | 0.004 | 2.58G | 0.030 | 4.62× | 7.73× |
| 100,000 | 0.071 | 1.42G | 0.045 | 2.21G | 0.071 | 1.01× | 1.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.106 | 1.54× |
| 1 | 5 | 0.238 | 0.415 | 1.74× |
| 1 | 10 | 0.420 | 0.834 | 1.99× |
| 10 | 1 | 0.044 | 0.088 | 2.02× |
| 10 | 5 | 0.182 | 0.446 | 2.45× |
| 10 | 10 | 0.386 | 0.939 | 2.43× |
| 100 | 1 | 0.042 | 0.084 | 2.01× |
| 100 | 5 | 0.195 | 0.413 | 2.12× |
| 100 | 10 | 0.394 | 0.859 | 2.18× |
| 1,000 | 1 | 0.040 | 0.099 | 2.50× |
| 1,000 | 5 | 0.186 | 0.466 | 2.51× |
| 1,000 | 10 | 0.400 | 0.939 | 2.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
