# Drawdown benchmark (`drawdown from cumulative maximum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.44M | 0.005 | 215.08M | 0.023 | 3.95× | 4.84× |
| 10,000 | 0.040 | 248.47M | 0.038 | 265.82M | 0.062 | 1.54× | 1.64× |
| 100,000 | 0.387 | 258.38M | 0.351 | 284.54M | 0.421 | 1.09× | 1.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.090 | 1.16× |
| 1 | 5 | 0.233 | 0.386 | 1.66× |
| 1 | 10 | 0.435 | 0.717 | 1.65× |
| 10 | 1 | 0.041 | 0.072 | 1.76× |
| 10 | 5 | 0.184 | 0.363 | 1.98× |
| 10 | 10 | 0.399 | 0.795 | 1.99× |
| 100 | 1 | 0.045 | 0.076 | 1.68× |
| 100 | 5 | 0.176 | 0.343 | 1.95× |
| 100 | 10 | 0.414 | 0.714 | 1.72× |
| 1,000 | 1 | 0.047 | 0.084 | 1.78× |
| 1,000 | 5 | 0.263 | 0.433 | 1.64× |
| 1,000 | 10 | 0.445 | 0.971 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
