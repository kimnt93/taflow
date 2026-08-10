# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.84M | 0.036 | 27.63M | 0.050 | 1.45× | 1.39× |
| 10,000 | 0.342 | 29.20M | 0.340 | 29.45M | 0.171 | 0.50× | 0.50× |
| 100,000 | 3.776 | 26.48M | 3.456 | 28.93M | 1.314 | 0.35× | 0.38× |
| 1,000,000 | 34.107 | 29.32M | 33.036 | 30.27M | 12.434 | 0.36× | 0.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.107 | 1.27× |
| 1 | 5 | 0.329 | 0.539 | 1.64× |
| 1 | 10 | 0.512 | 1.009 | 1.97× |
| 10 | 1 | 0.053 | 0.100 | 1.89× |
| 10 | 5 | 0.305 | 0.571 | 1.87× |
| 10 | 10 | 0.537 | 1.015 | 1.89× |
| 100 | 1 | 0.059 | 0.098 | 1.65× |
| 100 | 5 | 0.245 | 0.607 | 2.48× |
| 100 | 10 | 0.640 | 1.331 | 2.08× |
| 1,000 | 1 | 0.087 | 0.113 | 1.31× |
| 1,000 | 5 | 0.365 | 0.695 | 1.91× |
| 1,000 | 10 | 0.618 | 1.186 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
