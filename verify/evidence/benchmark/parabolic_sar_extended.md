# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.30M | 0.008 | 132.54M | 0.048 | 5.59× | 6.43× |
| 10,000 | 0.081 | 123.74M | 0.079 | 126.53M | 0.090 | 1.11× | 1.14× |
| 100,000 | 0.912 | 109.59M | 0.902 | 110.88M | 0.613 | 0.67× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.135 | 1.49× |
| 1 | 5 | 0.354 | 0.609 | 1.72× |
| 1 | 10 | 0.397 | 1.144 | 2.88× |
| 10 | 1 | 0.043 | 0.113 | 2.62× |
| 10 | 5 | 0.183 | 0.540 | 2.96× |
| 10 | 10 | 0.395 | 1.077 | 2.73× |
| 100 | 1 | 0.040 | 0.104 | 2.59× |
| 100 | 5 | 0.185 | 0.534 | 2.88× |
| 100 | 10 | 0.412 | 1.119 | 2.72× |
| 1,000 | 1 | 0.050 | 0.114 | 2.28× |
| 1,000 | 5 | 0.201 | 0.571 | 2.84× |
| 1,000 | 10 | 0.399 | 1.164 | 2.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
