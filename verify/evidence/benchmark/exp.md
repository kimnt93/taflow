# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 154.51M | 0.005 | 184.03M | 0.033 | 5.06× | 6.03× |
| 10,000 | 0.050 | 201.69M | 0.046 | 215.41M | 0.074 | 1.50× | 1.60× |
| 100,000 | 0.479 | 208.63M | 0.463 | 215.79M | 0.469 | 0.98× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.138 | 1.78× |
| 1 | 5 | 0.186 | 0.445 | 2.39× |
| 1 | 10 | 0.360 | 0.921 | 2.56× |
| 10 | 1 | 0.042 | 0.087 | 2.05× |
| 10 | 5 | 0.174 | 0.406 | 2.33× |
| 10 | 10 | 0.390 | 0.865 | 2.22× |
| 100 | 1 | 0.041 | 0.102 | 2.47× |
| 100 | 5 | 0.211 | 0.450 | 2.13× |
| 100 | 10 | 0.392 | 0.918 | 2.34× |
| 1,000 | 1 | 0.049 | 0.096 | 1.94× |
| 1,000 | 5 | 0.216 | 0.452 | 2.09× |
| 1,000 | 10 | 0.460 | 0.957 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
