# MathCbrt benchmark (`numpy.cbrt` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.23M | 0.018 | 55.77M | 0.026 | 1.44× | 1.46× |
| 10,000 | 0.161 | 62.14M | 0.166 | 60.06M | 0.155 | 0.96× | 0.93× |
| 100,000 | 1.769 | 56.53M | 1.657 | 60.35M | 1.359 | 0.77× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.069 | 0.87× |
| 1 | 5 | 0.289 | 0.324 | 1.12× |
| 1 | 10 | 0.395 | 0.570 | 1.44× |
| 10 | 1 | 0.040 | 0.053 | 1.34× |
| 10 | 5 | 0.171 | 0.269 | 1.57× |
| 10 | 10 | 0.404 | 0.597 | 1.48× |
| 100 | 1 | 0.043 | 0.061 | 1.40× |
| 100 | 5 | 0.188 | 0.279 | 1.48× |
| 100 | 10 | 0.391 | 0.585 | 1.49× |
| 1,000 | 1 | 0.062 | 0.073 | 1.17× |
| 1,000 | 5 | 0.188 | 0.305 | 1.62× |
| 1,000 | 10 | 0.400 | 0.756 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
