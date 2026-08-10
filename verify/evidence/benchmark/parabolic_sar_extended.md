# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.85M | 0.014 | 69.46M | 0.067 | 4.26× | 4.64× |
| 10,000 | 0.121 | 82.57M | 0.121 | 82.31M | 0.111 | 0.91× | 0.91× |
| 100,000 | 1.280 | 78.11M | 1.242 | 80.54M | 0.725 | 0.57× | 0.58× |
| 1,000,000 | 12.770 | 78.31M | 12.294 | 81.34M | 6.589 | 0.52× | 0.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.137 | 1.10× |
| 1 | 5 | 0.351 | 0.601 | 1.71× |
| 1 | 10 | 0.509 | 1.272 | 2.50× |
| 10 | 1 | 0.078 | 0.151 | 1.93× |
| 10 | 5 | 0.277 | 0.573 | 2.07× |
| 10 | 10 | 0.501 | 1.228 | 2.45× |
| 100 | 1 | 0.070 | 0.124 | 1.79× |
| 100 | 5 | 0.270 | 0.635 | 2.35× |
| 100 | 10 | 0.540 | 1.177 | 2.18× |
| 1,000 | 1 | 0.071 | 0.137 | 1.92× |
| 1,000 | 5 | 0.328 | 0.744 | 2.27× |
| 1,000 | 10 | 0.568 | 1.224 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
