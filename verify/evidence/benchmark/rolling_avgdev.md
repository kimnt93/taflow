# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.02M | 0.019 | 52.30M | 0.049 | 2.51× | 2.57× |
| 10,000 | 0.173 | 57.68M | 0.169 | 59.33M | 0.182 | 1.05× | 1.08× |
| 100,000 | 1.696 | 58.95M | 2.391 | 41.82M | 1.609 | 0.95× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.123 | 1.03× |
| 1 | 5 | 0.354 | 0.590 | 1.67× |
| 1 | 10 | 0.511 | 0.978 | 1.92× |
| 10 | 1 | 0.047 | 0.093 | 1.99× |
| 10 | 5 | 0.242 | 0.453 | 1.87× |
| 10 | 10 | 0.555 | 1.087 | 1.96× |
| 100 | 1 | 0.054 | 0.092 | 1.71× |
| 100 | 5 | 0.287 | 0.520 | 1.81× |
| 100 | 10 | 0.571 | 1.132 | 1.98× |
| 1,000 | 1 | 0.070 | 0.117 | 1.66× |
| 1,000 | 5 | 0.271 | 0.574 | 2.11× |
| 1,000 | 10 | 0.535 | 1.225 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
