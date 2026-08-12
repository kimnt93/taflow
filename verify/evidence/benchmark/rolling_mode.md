# RollingMode benchmark (`rolling mode` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.227 | 4.40M | 0.216 | 4.63M | 0.049 | 0.22× | 0.23× |
| 10,000 | 2.274 | 4.40M | 2.122 | 4.71M | 0.120 | 0.05× | 0.06× |
| 100,000 | 21.263 | 4.70M | 21.675 | 4.61M | 0.969 | 0.05× | 0.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.114 | 1.07× |
| 1 | 5 | 0.323 | 0.509 | 1.57× |
| 1 | 10 | 0.524 | 0.887 | 1.69× |
| 10 | 1 | 0.052 | 0.085 | 1.64× |
| 10 | 5 | 0.240 | 0.418 | 1.74× |
| 10 | 10 | 0.502 | 0.891 | 1.77× |
| 100 | 1 | 0.075 | 0.112 | 1.50× |
| 100 | 5 | 0.256 | 0.529 | 2.06× |
| 100 | 10 | 0.505 | 1.145 | 2.27× |
| 1,000 | 1 | 0.299 | 0.166 | 0.56× |
| 1,000 | 5 | 0.457 | 0.773 | 1.69× |
| 1,000 | 10 | 0.747 | 1.589 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
