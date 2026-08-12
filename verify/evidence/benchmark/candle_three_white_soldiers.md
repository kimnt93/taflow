# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.45M | 0.018 | 55.00M | 0.043 | 2.02× | 2.34× |
| 10,000 | 0.167 | 60.02M | 0.162 | 61.88M | 0.180 | 1.08× | 1.12× |
| 100,000 | 1.639 | 61.01M | 1.586 | 63.03M | 1.585 | 0.97× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.129 | 1.32× |
| 1 | 5 | 0.372 | 0.457 | 1.23× |
| 1 | 10 | 0.542 | 0.907 | 1.67× |
| 10 | 1 | 0.059 | 0.085 | 1.43× |
| 10 | 5 | 0.258 | 0.431 | 1.67× |
| 10 | 10 | 0.513 | 0.903 | 1.76× |
| 100 | 1 | 0.057 | 0.090 | 1.58× |
| 100 | 5 | 0.254 | 0.445 | 1.75× |
| 100 | 10 | 0.544 | 0.915 | 1.68× |
| 1,000 | 1 | 0.072 | 0.104 | 1.44× |
| 1,000 | 5 | 0.302 | 0.537 | 1.78× |
| 1,000 | 10 | 0.572 | 1.085 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
