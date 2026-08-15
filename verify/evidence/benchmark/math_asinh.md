# MathAsinh benchmark (`numpy.arcsinh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.14M | 0.015 | 68.56M | 0.023 | 1.67× | 1.61× |
| 10,000 | 0.132 | 75.66M | 0.128 | 77.90M | 0.141 | 1.07× | 1.10× |
| 100,000 | 1.369 | 73.06M | 1.263 | 79.17M | 1.293 | 0.94× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.097 | 1.32× |
| 1 | 5 | 0.218 | 0.284 | 1.31× |
| 1 | 10 | 0.376 | 0.567 | 1.51× |
| 10 | 1 | 0.042 | 0.061 | 1.43× |
| 10 | 5 | 0.189 | 0.282 | 1.49× |
| 10 | 10 | 0.415 | 0.594 | 1.43× |
| 100 | 1 | 0.044 | 0.061 | 1.39× |
| 100 | 5 | 0.185 | 0.271 | 1.46× |
| 100 | 10 | 0.373 | 0.578 | 1.55× |
| 1,000 | 1 | 0.055 | 0.073 | 1.32× |
| 1,000 | 5 | 0.192 | 0.310 | 1.61× |
| 1,000 | 10 | 0.404 | 0.768 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
