# CenterOfGravity benchmark (`CenterOfGravity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.72M | 0.018 | 54.35M | 0.167 | 8.63× | 9.07× |
| 10,000 | 0.182 | 54.91M | 0.181 | 55.10M | 0.607 | 3.33× | 3.35× |
| 100,000 | 1.838 | 54.40M | 1.883 | 53.11M | 9.420 | 5.12× | 5.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 1.088 | 0.273 | 0.25× |
| 1 | 5 | 0.273 | 1.323 | 4.85× |
| 1 | 10 | 0.595 | 2.559 | 4.30× |
| 10 | 1 | 0.043 | 0.186 | 4.30× |
| 10 | 5 | 0.210 | 1.224 | 5.84× |
| 10 | 10 | 0.544 | 2.586 | 4.76× |
| 100 | 1 | 0.054 | 0.220 | 4.06× |
| 100 | 5 | 0.243 | 1.042 | 4.30× |
| 100 | 10 | 0.430 | 2.248 | 5.23× |
| 1,000 | 1 | 0.064 | 0.243 | 3.78× |
| 1,000 | 5 | 0.235 | 1.188 | 5.05× |
| 1,000 | 10 | 0.478 | 2.770 | 5.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
