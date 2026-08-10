# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.78M | 0.014 | 69.85M | 0.044 | 2.55× | 3.08× |
| 10,000 | 0.169 | 59.06M | 0.201 | 49.66M | 0.198 | 1.17× | 0.98× |
| 100,000 | 1.694 | 59.04M | 1.646 | 60.77M | 1.696 | 1.00× | 1.03× |
| 1,000,000 | 17.059 | 58.62M | 17.240 | 58.00M | 16.967 | 0.99× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.118 | 1.64× |
| 1 | 5 | 0.286 | 0.473 | 1.65× |
| 1 | 10 | 0.489 | 0.960 | 1.96× |
| 10 | 1 | 0.052 | 0.094 | 1.81× |
| 10 | 5 | 0.247 | 0.455 | 1.84× |
| 10 | 10 | 0.506 | 0.915 | 1.81× |
| 100 | 1 | 0.053 | 0.104 | 1.96× |
| 100 | 5 | 0.286 | 0.495 | 1.73× |
| 100 | 10 | 0.534 | 1.002 | 1.88× |
| 1,000 | 1 | 0.062 | 0.105 | 1.70× |
| 1,000 | 5 | 0.279 | 0.586 | 2.10× |
| 1,000 | 10 | 0.641 | 1.124 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
