# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.86M | 0.005 | 195.61M | 0.032 | 5.39× | 6.36× |
| 10,000 | 0.047 | 214.57M | 0.043 | 234.94M | 0.070 | 1.49× | 1.63× |
| 100,000 | 0.455 | 219.76M | 0.462 | 216.54M | 0.416 | 0.91× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.135 | 1.93× |
| 1 | 5 | 0.231 | 0.425 | 1.84× |
| 1 | 10 | 0.431 | 0.951 | 2.21× |
| 10 | 1 | 0.042 | 0.085 | 2.01× |
| 10 | 5 | 0.176 | 0.460 | 2.61× |
| 10 | 10 | 0.381 | 0.963 | 2.53× |
| 100 | 1 | 0.050 | 0.093 | 1.87× |
| 100 | 5 | 0.186 | 0.424 | 2.28× |
| 100 | 10 | 0.407 | 0.902 | 2.22× |
| 1,000 | 1 | 0.048 | 0.096 | 1.98× |
| 1,000 | 5 | 0.227 | 0.494 | 2.17× |
| 1,000 | 10 | 0.524 | 1.036 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
