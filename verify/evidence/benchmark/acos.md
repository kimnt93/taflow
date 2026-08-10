# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.47M | 0.009 | 116.49M | 0.033 | 3.53× | 3.87× |
| 10,000 | 0.067 | 148.63M | 0.064 | 157.25M | 0.092 | 1.36× | 1.44× |
| 100,000 | 0.654 | 152.99M | 0.630 | 158.78M | 0.665 | 1.02× | 1.06× |
| 1,000,000 | 6.795 | 147.16M | 6.864 | 145.68M | 6.420 | 0.94× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.107 | 1.32× |
| 1 | 5 | 0.308 | 0.461 | 1.50× |
| 1 | 10 | 0.468 | 0.895 | 1.91× |
| 10 | 1 | 0.047 | 0.096 | 2.05× |
| 10 | 5 | 0.233 | 0.417 | 1.79× |
| 10 | 10 | 0.475 | 0.918 | 1.93× |
| 100 | 1 | 0.055 | 0.092 | 1.68× |
| 100 | 5 | 0.232 | 0.417 | 1.80× |
| 100 | 10 | 0.479 | 0.838 | 1.75× |
| 1,000 | 1 | 0.062 | 0.096 | 1.54× |
| 1,000 | 5 | 0.241 | 0.457 | 1.89× |
| 1,000 | 10 | 0.503 | 0.941 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
