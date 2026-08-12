# InsideBar benchmark (`inside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.70M | 0.007 | 148.71M | 0.024 | 3.04× | 3.57× |
| 10,000 | 0.036 | 281.00M | 0.031 | 319.75M | 0.043 | 1.22× | 1.39× |
| 100,000 | 0.300 | 332.99M | 0.285 | 350.43M | 0.252 | 0.84× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.109 | 1.45× |
| 1 | 5 | 0.351 | 0.362 | 1.03× |
| 1 | 10 | 0.480 | 0.766 | 1.60× |
| 10 | 1 | 0.058 | 0.087 | 1.50× |
| 10 | 5 | 0.248 | 0.370 | 1.49× |
| 10 | 10 | 0.483 | 0.739 | 1.53× |
| 100 | 1 | 0.048 | 0.068 | 1.41× |
| 100 | 5 | 0.216 | 0.344 | 1.59× |
| 100 | 10 | 0.529 | 0.730 | 1.38× |
| 1,000 | 1 | 0.054 | 0.075 | 1.37× |
| 1,000 | 5 | 0.248 | 0.509 | 2.06× |
| 1,000 | 10 | 0.486 | 1.216 | 2.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
