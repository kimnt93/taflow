# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.82M | 0.013 | 77.52M | 0.042 | 3.14× | 3.25× |
| 10,000 | 0.157 | 63.72M | 0.154 | 65.05M | 0.182 | 1.16× | 1.19× |
| 100,000 | 1.585 | 63.07M | 1.528 | 65.45M | 1.611 | 1.02× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.141 | 1.46× |
| 1 | 5 | 0.447 | 0.466 | 1.04× |
| 1 | 10 | 0.473 | 0.912 | 1.93× |
| 10 | 1 | 0.049 | 0.089 | 1.84× |
| 10 | 5 | 0.258 | 0.531 | 2.06× |
| 10 | 10 | 0.485 | 0.927 | 1.91× |
| 100 | 1 | 0.050 | 0.092 | 1.83× |
| 100 | 5 | 0.237 | 0.453 | 1.91× |
| 100 | 10 | 0.555 | 1.001 | 1.80× |
| 1,000 | 1 | 0.073 | 0.113 | 1.55× |
| 1,000 | 5 | 0.267 | 0.592 | 2.22× |
| 1,000 | 10 | 0.611 | 1.175 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
