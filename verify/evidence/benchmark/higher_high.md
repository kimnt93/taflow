# HigherHigh benchmark (`higher high relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.82M | 0.006 | 161.45M | 0.017 | 2.20× | 2.68× |
| 10,000 | 0.036 | 274.98M | 0.033 | 303.51M | 0.024 | 0.67× | 0.74× |
| 100,000 | 0.297 | 336.52M | 0.280 | 357.02M | 0.110 | 0.37× | 0.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.096 | 0.87× |
| 1 | 5 | 0.271 | 0.331 | 1.22× |
| 1 | 10 | 0.481 | 0.735 | 1.53× |
| 10 | 1 | 0.051 | 0.068 | 1.35× |
| 10 | 5 | 0.235 | 0.330 | 1.41× |
| 10 | 10 | 0.493 | 0.688 | 1.40× |
| 100 | 1 | 0.054 | 0.063 | 1.18× |
| 100 | 5 | 0.266 | 0.382 | 1.43× |
| 100 | 10 | 0.476 | 0.692 | 1.45× |
| 1,000 | 1 | 0.057 | 0.067 | 1.17× |
| 1,000 | 5 | 0.233 | 0.373 | 1.60× |
| 1,000 | 10 | 0.583 | 0.852 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
