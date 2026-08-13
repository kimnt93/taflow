# MathAbs benchmark (`numpy.abs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.80M | 0.019 | 51.85M | 0.012 | 0.51× | 0.62× |
| 10,000 | 0.139 | 72.18M | 0.133 | 75.09M | 0.015 | 0.11× | 0.11× |
| 100,000 | 1.297 | 77.11M | 1.275 | 78.43M | 0.039 | 0.03× | 0.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.074 | 0.83× |
| 1 | 5 | 0.369 | 0.302 | 0.82× |
| 1 | 10 | 0.578 | 0.577 | 1.00× |
| 10 | 1 | 0.061 | 0.057 | 0.94× |
| 10 | 5 | 0.271 | 0.269 | 0.99× |
| 10 | 10 | 0.562 | 0.586 | 1.04× |
| 100 | 1 | 0.063 | 0.059 | 0.95× |
| 100 | 5 | 0.274 | 0.263 | 0.96× |
| 100 | 10 | 0.571 | 0.576 | 1.01× |
| 1,000 | 1 | 0.080 | 0.060 | 0.75× |
| 1,000 | 5 | 0.281 | 0.273 | 0.97× |
| 1,000 | 10 | 0.586 | 0.603 | 1.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
