# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 35.04M | 0.027 | 37.03M | 0.058 | 2.03× | 2.14× |
| 10,000 | 0.241 | 41.52M | 0.251 | 39.84M | 0.279 | 1.16× | 1.11× |
| 100,000 | 2.968 | 33.70M | 2.687 | 37.22M | 2.647 | 0.89× | 0.99× |
| 1,000,000 | 25.958 | 38.52M | 25.093 | 39.85M | 24.597 | 0.95× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.164 | 0.149 | 0.91× |
| 1 | 5 | 0.283 | 0.535 | 1.89× |
| 1 | 10 | 0.559 | 1.088 | 1.94× |
| 10 | 1 | 0.062 | 0.123 | 1.99× |
| 10 | 5 | 0.320 | 0.474 | 1.48× |
| 10 | 10 | 0.570 | 1.031 | 1.81× |
| 100 | 1 | 0.062 | 0.108 | 1.75× |
| 100 | 5 | 0.283 | 0.531 | 1.88× |
| 100 | 10 | 0.630 | 1.060 | 1.68× |
| 1,000 | 1 | 0.080 | 0.124 | 1.55× |
| 1,000 | 5 | 0.286 | 0.646 | 2.26× |
| 1,000 | 10 | 0.714 | 1.271 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
