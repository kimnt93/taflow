# RelativeMomentumIndex benchmark (`RMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.04M | 0.007 | 135.46M | 0.190 | 22.80× | 25.73× |
| 10,000 | 0.067 | 149.09M | 0.072 | 138.07M | 0.566 | 8.44× | 7.81× |
| 100,000 | 0.690 | 144.92M | 0.669 | 149.52M | 4.316 | 6.25× | 6.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.244 | 2.22× |
| 1 | 5 | 0.234 | 1.098 | 4.68× |
| 1 | 10 | 0.413 | 2.507 | 6.08× |
| 10 | 1 | 0.045 | 0.225 | 5.00× |
| 10 | 5 | 0.186 | 1.119 | 6.02× |
| 10 | 10 | 0.390 | 2.481 | 6.36× |
| 100 | 1 | 0.044 | 0.225 | 5.15× |
| 100 | 5 | 0.224 | 1.145 | 5.12× |
| 100 | 10 | 0.417 | 2.591 | 6.21× |
| 1,000 | 1 | 0.056 | 0.281 | 5.02× |
| 1,000 | 5 | 0.220 | 1.330 | 6.05× |
| 1,000 | 10 | 0.433 | 2.931 | 6.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
