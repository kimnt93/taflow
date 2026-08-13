# SuperSmoother benchmark (`SuperSmoother` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 33.92M | 0.025 | 39.52M | 0.145 | 4.92× | 5.73× |
| 10,000 | 0.181 | 55.11M | 0.183 | 54.53M | 0.417 | 2.30× | 2.27× |
| 100,000 | 1.667 | 60.01M | 1.804 | 55.43M | 3.518 | 2.11× | 1.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.230 | 2.42× |
| 1 | 5 | 0.421 | 1.020 | 2.42× |
| 1 | 10 | 0.743 | 3.085 | 4.15× |
| 10 | 1 | 0.117 | 0.485 | 4.15× |
| 10 | 5 | 0.391 | 1.132 | 2.90× |
| 10 | 10 | 0.681 | 2.376 | 3.49× |
| 100 | 1 | 0.081 | 0.227 | 2.79× |
| 100 | 5 | 0.377 | 1.184 | 3.14× |
| 100 | 10 | 0.687 | 2.537 | 3.69× |
| 1,000 | 1 | 0.102 | 0.250 | 2.45× |
| 1,000 | 5 | 0.362 | 1.247 | 3.44× |
| 1,000 | 10 | 0.702 | 3.535 | 5.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
