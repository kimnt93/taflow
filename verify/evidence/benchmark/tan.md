# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 24.00M | 0.037 | 26.87M | 0.044 | 1.04× | 1.17× |
| 10,000 | 0.315 | 31.75M | 0.330 | 30.33M | 0.211 | 0.67× | 0.64× |
| 100,000 | 2.956 | 33.83M | 3.142 | 31.82M | 2.141 | 0.72× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.163 | 1.23× |
| 1 | 5 | 0.349 | 0.444 | 1.27× |
| 1 | 10 | 0.589 | 0.904 | 1.54× |
| 10 | 1 | 0.065 | 0.083 | 1.28× |
| 10 | 5 | 0.272 | 0.420 | 1.54× |
| 10 | 10 | 0.604 | 0.884 | 1.46× |
| 100 | 1 | 0.069 | 0.085 | 1.24× |
| 100 | 5 | 0.294 | 0.434 | 1.48× |
| 100 | 10 | 0.636 | 0.906 | 1.43× |
| 1,000 | 1 | 0.098 | 0.105 | 1.07× |
| 1,000 | 5 | 0.296 | 0.516 | 1.74× |
| 1,000 | 10 | 0.594 | 1.092 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
