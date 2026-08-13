# ArmsIndex benchmark (`Trin` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 19.90M | 0.042 | 24.08M | 8.572 | 170.60× | 206.41× |
| 10,000 | 0.330 | 30.29M | 0.322 | 31.05M | 85.717 | 259.61× | 266.16× |
| 100,000 | 2.980 | 33.55M | 3.014 | 33.18M | 839.403 | 281.64× | 278.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.176 | 0.279 | 1.58× |
| 1 | 5 | 0.360 | 1.233 | 3.42× |
| 1 | 10 | 0.640 | 2.435 | 3.81× |
| 10 | 1 | 0.073 | 0.301 | 4.14× |
| 10 | 5 | 0.312 | 1.478 | 4.73× |
| 10 | 10 | 0.652 | 3.251 | 4.99× |
| 100 | 1 | 0.079 | 1.069 | 13.51× |
| 100 | 5 | 0.314 | 5.473 | 17.43× |
| 100 | 10 | 0.621 | 11.185 | 18.00× |
| 1,000 | 1 | 0.112 | 8.453 | 75.50× |
| 1,000 | 5 | 0.448 | 46.312 | 103.40× |
| 1,000 | 10 | 0.943 | 93.233 | 98.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
