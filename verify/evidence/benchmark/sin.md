# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.49M | 0.013 | 75.11M | 0.039 | 2.96× | 2.91× |
| 10,000 | 0.156 | 63.99M | 0.159 | 62.87M | 0.178 | 1.14× | 1.12× |
| 100,000 | 1.569 | 63.72M | 1.618 | 61.82M | 1.632 | 1.04× | 1.01× |
| 1,000,000 | 17.652 | 56.65M | 17.050 | 58.65M | 16.812 | 0.95× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.117 | 1.54× |
| 1 | 5 | 0.223 | 0.436 | 1.95× |
| 1 | 10 | 0.465 | 0.943 | 2.03× |
| 10 | 1 | 0.050 | 0.081 | 1.60× |
| 10 | 5 | 0.213 | 0.437 | 2.05× |
| 10 | 10 | 0.483 | 0.913 | 1.89× |
| 100 | 1 | 0.061 | 0.091 | 1.50× |
| 100 | 5 | 0.240 | 0.443 | 1.84× |
| 100 | 10 | 0.517 | 1.011 | 1.96× |
| 1,000 | 1 | 0.069 | 0.100 | 1.46× |
| 1,000 | 5 | 0.264 | 0.606 | 2.30× |
| 1,000 | 10 | 0.550 | 1.089 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
