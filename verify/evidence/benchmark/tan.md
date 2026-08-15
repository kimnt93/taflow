# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.82M | 0.014 | 69.29M | 0.043 | 2.67× | 2.94× |
| 10,000 | 0.193 | 51.82M | 0.187 | 53.51M | 0.214 | 1.11× | 1.15× |
| 100,000 | 1.898 | 52.69M | 1.922 | 52.03M | 1.848 | 0.97× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.127 | 0.86× |
| 1 | 5 | 0.206 | 0.444 | 2.15× |
| 1 | 10 | 0.395 | 0.857 | 2.17× |
| 10 | 1 | 0.043 | 0.085 | 1.99× |
| 10 | 5 | 0.181 | 0.416 | 2.30× |
| 10 | 10 | 0.367 | 0.857 | 2.34× |
| 100 | 1 | 0.046 | 0.088 | 1.92× |
| 100 | 5 | 0.184 | 0.418 | 2.27× |
| 100 | 10 | 0.401 | 0.842 | 2.10× |
| 1,000 | 1 | 0.079 | 0.103 | 1.30× |
| 1,000 | 5 | 0.210 | 0.519 | 2.47× |
| 1,000 | 10 | 0.409 | 1.118 | 2.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
