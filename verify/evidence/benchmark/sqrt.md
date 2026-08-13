# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.65M | 0.019 | 51.95M | 0.027 | 1.08× | 1.39× |
| 10,000 | 0.144 | 69.23M | 0.141 | 70.76M | 0.040 | 0.27× | 0.28× |
| 100,000 | 1.339 | 74.69M | 1.312 | 76.22M | 0.159 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.114 | 1.20× |
| 1 | 5 | 0.450 | 0.455 | 1.01× |
| 1 | 10 | 0.580 | 0.860 | 1.48× |
| 10 | 1 | 0.061 | 0.086 | 1.40× |
| 10 | 5 | 0.272 | 0.390 | 1.43× |
| 10 | 10 | 0.570 | 0.855 | 1.50× |
| 100 | 1 | 0.066 | 0.085 | 1.29× |
| 100 | 5 | 0.281 | 0.398 | 1.42× |
| 100 | 10 | 0.559 | 0.872 | 1.56× |
| 1,000 | 1 | 0.071 | 0.087 | 1.22× |
| 1,000 | 5 | 0.293 | 0.435 | 1.48× |
| 1,000 | 10 | 0.599 | 0.887 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
