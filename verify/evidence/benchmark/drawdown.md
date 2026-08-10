# Drawdown benchmark (`drawdown from cumulative maximum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.45M | 0.007 | 147.82M | 0.024 | 3.16× | 3.48× |
| 10,000 | 0.044 | 226.42M | 0.042 | 240.86M | 0.059 | 1.33× | 1.41× |
| 100,000 | 0.392 | 255.07M | 0.369 | 271.19M | 0.406 | 1.03× | 1.10× |
| 1,000,000 | 4.241 | 235.78M | 3.672 | 272.31M | 5.170 | 1.22× | 1.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.120 | 1.14× |
| 1 | 5 | 0.334 | 0.370 | 1.11× |
| 1 | 10 | 0.446 | 0.696 | 1.56× |
| 10 | 1 | 0.047 | 0.073 | 1.56× |
| 10 | 5 | 0.216 | 0.338 | 1.56× |
| 10 | 10 | 0.468 | 0.710 | 1.52× |
| 100 | 1 | 0.047 | 0.076 | 1.61× |
| 100 | 5 | 0.221 | 0.353 | 1.60× |
| 100 | 10 | 0.458 | 0.722 | 1.58× |
| 1,000 | 1 | 0.052 | 0.080 | 1.54× |
| 1,000 | 5 | 0.222 | 0.406 | 1.83× |
| 1,000 | 10 | 0.509 | 0.968 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
