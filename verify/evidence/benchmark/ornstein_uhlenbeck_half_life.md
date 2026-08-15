# OrnsteinUhlenbeckHalfLife benchmark (`rolling OU half life` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.41M | 0.043 | 23.27M | 0.282 | 6.03× | 6.56× |
| 10,000 | 0.440 | 22.70M | 0.453 | 22.07M | 1.499 | 3.40× | 3.31× |
| 100,000 | 4.447 | 22.49M | 4.423 | 22.61M | 18.031 | 4.05× | 4.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.118 | 1.46× |
| 1 | 5 | 0.333 | 0.433 | 1.30× |
| 1 | 10 | 0.385 | 0.833 | 2.16× |
| 10 | 1 | 0.040 | 0.085 | 2.11× |
| 10 | 5 | 0.185 | 0.399 | 2.16× |
| 10 | 10 | 0.382 | 0.830 | 2.17× |
| 100 | 1 | 0.049 | 0.252 | 5.16× |
| 100 | 5 | 0.200 | 1.254 | 6.26× |
| 100 | 10 | 0.440 | 2.587 | 5.87× |
| 1,000 | 1 | 0.099 | 0.388 | 3.93× |
| 1,000 | 5 | 0.235 | 1.608 | 6.84× |
| 1,000 | 10 | 0.469 | 3.539 | 7.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
