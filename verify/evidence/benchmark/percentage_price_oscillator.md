# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.98M | 0.005 | 190.58M | 0.040 | 6.56× | 7.57× |
| 10,000 | 0.042 | 240.09M | 0.038 | 260.89M | 0.081 | 1.95× | 2.12× |
| 100,000 | 0.389 | 256.77M | 0.359 | 278.92M | 0.490 | 1.26× | 1.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.162 | 2.20× |
| 1 | 5 | 0.322 | 0.511 | 1.59× |
| 1 | 10 | 0.432 | 0.970 | 2.25× |
| 10 | 1 | 0.047 | 0.095 | 2.03× |
| 10 | 5 | 0.187 | 0.492 | 2.63× |
| 10 | 10 | 0.457 | 1.041 | 2.28× |
| 100 | 1 | 0.047 | 0.096 | 2.05× |
| 100 | 5 | 0.193 | 0.492 | 2.54× |
| 100 | 10 | 0.399 | 1.019 | 2.55× |
| 1,000 | 1 | 0.053 | 0.099 | 1.87× |
| 1,000 | 5 | 0.196 | 0.473 | 2.41× |
| 1,000 | 10 | 0.424 | 1.038 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
