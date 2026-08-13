# MathCbrt benchmark (`numpy.cbrt` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.75M | 0.042 | 23.76M | 0.025 | 0.54× | 0.59× |
| 10,000 | 0.353 | 28.30M | 0.352 | 28.38M | 0.147 | 0.42× | 0.42× |
| 100,000 | 3.416 | 29.27M | 3.358 | 29.78M | 1.372 | 0.40× | 0.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.082 | 0.68× |
| 1 | 5 | 0.425 | 0.327 | 0.77× |
| 1 | 10 | 0.589 | 0.577 | 0.98× |
| 10 | 1 | 0.061 | 0.058 | 0.95× |
| 10 | 5 | 0.271 | 0.280 | 1.03× |
| 10 | 10 | 0.570 | 0.564 | 0.99× |
| 100 | 1 | 0.063 | 0.063 | 0.99× |
| 100 | 5 | 0.271 | 0.278 | 1.03× |
| 100 | 10 | 0.608 | 0.578 | 0.95× |
| 1,000 | 1 | 0.098 | 0.069 | 0.70× |
| 1,000 | 5 | 0.287 | 0.288 | 1.00× |
| 1,000 | 10 | 0.611 | 0.726 | 1.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
