# MathDegrees benchmark (`numpy.degrees` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.98M | 0.019 | 52.66M | 0.013 | 0.56× | 0.68× |
| 10,000 | 0.139 | 72.18M | 0.130 | 77.18M | 0.024 | 0.18× | 0.19× |
| 100,000 | 1.245 | 80.32M | 1.210 | 82.62M | 0.128 | 0.10× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.217 | 0.090 | 0.41× |
| 1 | 5 | 0.465 | 0.279 | 0.60× |
| 1 | 10 | 0.547 | 0.562 | 1.03× |
| 10 | 1 | 0.060 | 0.054 | 0.90× |
| 10 | 5 | 0.298 | 0.272 | 0.91× |
| 10 | 10 | 0.585 | 0.574 | 0.98× |
| 100 | 1 | 0.062 | 0.052 | 0.85× |
| 100 | 5 | 0.268 | 0.287 | 1.07× |
| 100 | 10 | 0.558 | 0.550 | 0.99× |
| 1,000 | 1 | 0.072 | 0.056 | 0.79× |
| 1,000 | 5 | 0.282 | 0.282 | 1.00× |
| 1,000 | 10 | 0.586 | 0.617 | 1.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
