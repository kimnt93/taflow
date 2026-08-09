# MathCot benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.60M | 0.023 | 44.14M | 0.021 | 0.93× | 0.92× |
| 10,000 | 0.208 | 48.09M | 0.206 | 48.60M | 0.226 | 1.09× | 1.10× |
| 100,000 | 3.021 | 33.10M | 2.486 | 40.23M | 2.151 | 0.71× | 0.87× |
| 1,000,000 | 23.355 | 42.82M | 23.276 | 42.96M | 21.413 | 0.92× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.069 | 0.87× |
| 1 | 5 | 0.254 | 0.225 | 0.88× |
| 1 | 10 | 0.579 | 0.506 | 0.87× |
| 10 | 1 | 0.047 | 0.042 | 0.90× |
| 10 | 5 | 0.253 | 0.245 | 0.97× |
| 10 | 10 | 0.504 | 0.487 | 0.97× |
| 100 | 1 | 0.064 | 0.057 | 0.89× |
| 100 | 5 | 0.263 | 0.221 | 0.84× |
| 100 | 10 | 0.520 | 0.471 | 0.91× |
| 1,000 | 1 | 0.077 | 0.070 | 0.90× |
| 1,000 | 5 | 0.277 | 0.314 | 1.13× |
| 1,000 | 10 | 0.656 | 0.572 | 0.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
