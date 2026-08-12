# Crossover benchmark (`causal crossover` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.27M | 0.006 | 154.77M | 0.022 | 2.78× | 3.35× |
| 10,000 | 0.038 | 265.98M | 0.035 | 288.55M | 0.029 | 0.78× | 0.84× |
| 100,000 | 0.358 | 279.26M | 0.286 | 349.77M | 0.159 | 0.44× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.084 | 1.23× |
| 1 | 5 | 0.256 | 0.324 | 1.26× |
| 1 | 10 | 0.476 | 0.668 | 1.40× |
| 10 | 1 | 0.049 | 0.064 | 1.30× |
| 10 | 5 | 0.240 | 0.333 | 1.39× |
| 10 | 10 | 0.500 | 0.693 | 1.39× |
| 100 | 1 | 0.048 | 0.068 | 1.42× |
| 100 | 5 | 0.234 | 0.318 | 1.36× |
| 100 | 10 | 0.479 | 0.724 | 1.51× |
| 1,000 | 1 | 0.055 | 0.066 | 1.21× |
| 1,000 | 5 | 0.243 | 0.385 | 1.58× |
| 1,000 | 10 | 0.485 | 1.019 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
