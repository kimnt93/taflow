# MovingAverageEnvelope benchmark (`MaEnvelope` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.72M | 0.024 | 41.44M | 0.540 | 23.60× | 22.37× |
| 10,000 | 0.191 | 52.23M | 0.178 | 56.28M | 5.151 | 26.91× | 28.99× |
| 100,000 | 2.265 | 44.16M | 1.803 | 55.46M | 44.443 | 19.63× | 24.65× |
| 1,000,000 | 21.802 | 45.87M | 19.456 | 51.40M | 460.219 | 21.11× | 23.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.338 | 4.44× |
| 1 | 5 | 0.294 | 1.469 | 5.00× |
| 1 | 10 | 0.508 | 2.817 | 5.55× |
| 10 | 1 | 0.064 | 0.260 | 4.10× |
| 10 | 5 | 0.267 | 1.487 | 5.56× |
| 10 | 10 | 0.507 | 2.889 | 5.70× |
| 100 | 1 | 0.054 | 0.290 | 5.34× |
| 100 | 5 | 0.256 | 1.750 | 6.84× |
| 100 | 10 | 0.529 | 3.129 | 5.92× |
| 1,000 | 1 | 0.072 | 0.951 | 13.30× |
| 1,000 | 5 | 0.295 | 3.528 | 11.95× |
| 1,000 | 10 | 0.538 | 13.958 | 25.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
