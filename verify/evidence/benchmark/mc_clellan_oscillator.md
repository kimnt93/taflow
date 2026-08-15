# McClellanOscillator benchmark (`McClellanOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.88M | 0.006 | 175.69M | 8.282 | 1224.73× | 1455.10× |
| 10,000 | 0.051 | 194.51M | 0.048 | 208.00M | 83.121 | 1616.81× | 1728.95× |
| 100,000 | 0.463 | 216.01M | 0.433 | 231.05M | 862.417 | 1862.87× | 1992.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.279 | 1.67× |
| 1 | 5 | 0.213 | 1.345 | 6.30× |
| 1 | 10 | 0.399 | 2.174 | 5.44× |
| 10 | 1 | 0.048 | 0.295 | 6.18× |
| 10 | 5 | 0.205 | 1.815 | 8.85× |
| 10 | 10 | 0.453 | 2.965 | 6.55× |
| 100 | 1 | 0.044 | 1.083 | 24.35× |
| 100 | 5 | 0.216 | 5.761 | 26.68× |
| 100 | 10 | 0.425 | 11.160 | 26.25× |
| 1,000 | 1 | 0.057 | 9.031 | 157.14× |
| 1,000 | 5 | 0.324 | 46.685 | 144.04× |
| 1,000 | 10 | 0.543 | 90.910 | 167.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
