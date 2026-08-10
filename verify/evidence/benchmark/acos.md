# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.52M | 0.009 | 111.37M | 0.036 | 3.75× | 4.04× |
| 10,000 | 0.075 | 132.92M | 0.069 | 144.11M | 0.100 | 1.33× | 1.44× |
| 100,000 | 0.738 | 135.49M | 0.713 | 140.28M | 0.702 | 0.95× | 0.99× |
| 1,000,000 | 7.994 | 125.09M | 7.250 | 137.94M | 6.980 | 0.87× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.146 | 1.42× |
| 1 | 5 | 0.285 | 0.482 | 1.69× |
| 1 | 10 | 0.497 | 0.944 | 1.90× |
| 10 | 1 | 0.059 | 0.091 | 1.54× |
| 10 | 5 | 0.219 | 0.432 | 1.97× |
| 10 | 10 | 0.555 | 0.888 | 1.60× |
| 100 | 1 | 0.049 | 0.084 | 1.73× |
| 100 | 5 | 0.271 | 0.484 | 1.79× |
| 100 | 10 | 0.484 | 0.908 | 1.88× |
| 1,000 | 1 | 0.061 | 0.091 | 1.50× |
| 1,000 | 5 | 0.228 | 0.451 | 1.98× |
| 1,000 | 10 | 0.566 | 1.032 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
