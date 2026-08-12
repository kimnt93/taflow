# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.61M | 0.019 | 51.50M | 0.044 | 2.47× | 2.29× |
| 10,000 | 0.200 | 50.01M | 0.194 | 51.47M | 0.224 | 1.12× | 1.15× |
| 100,000 | 1.990 | 50.25M | 2.052 | 48.74M | 2.107 | 1.06× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.101 | 1.35× |
| 1 | 5 | 0.264 | 0.436 | 1.65× |
| 1 | 10 | 0.446 | 0.923 | 2.07× |
| 10 | 1 | 0.055 | 0.090 | 1.64× |
| 10 | 5 | 0.226 | 0.419 | 1.85× |
| 10 | 10 | 0.450 | 0.892 | 1.98× |
| 100 | 1 | 0.058 | 0.086 | 1.48× |
| 100 | 5 | 0.258 | 0.467 | 1.81× |
| 100 | 10 | 0.483 | 0.890 | 1.84× |
| 1,000 | 1 | 0.081 | 0.114 | 1.41× |
| 1,000 | 5 | 0.251 | 0.543 | 2.17× |
| 1,000 | 10 | 0.544 | 1.145 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
