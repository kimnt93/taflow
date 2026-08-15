# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 261.93M | 0.003 | 337.95M | 0.034 | 8.90× | 11.49× |
| 10,000 | 0.023 | 429.11M | 0.021 | 469.49M | 0.050 | 2.15× | 2.35× |
| 100,000 | 0.216 | 462.83M | 0.191 | 524.05M | 0.213 | 0.99× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.106 | 1.26× |
| 1 | 5 | 0.281 | 0.471 | 1.68× |
| 1 | 10 | 0.426 | 0.972 | 2.28× |
| 10 | 1 | 0.044 | 0.097 | 2.20× |
| 10 | 5 | 0.187 | 0.484 | 2.59× |
| 10 | 10 | 0.399 | 0.984 | 2.47× |
| 100 | 1 | 0.048 | 0.088 | 1.82× |
| 100 | 5 | 0.178 | 0.433 | 2.43× |
| 100 | 10 | 0.412 | 0.940 | 2.28× |
| 1,000 | 1 | 0.057 | 0.089 | 1.56× |
| 1,000 | 5 | 0.196 | 0.488 | 2.48× |
| 1,000 | 10 | 0.407 | 0.945 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
