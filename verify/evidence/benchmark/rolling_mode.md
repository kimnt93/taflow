# RollingMode benchmark (`rolling mode` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.234 | 4.28M | 0.228 | 4.39M | 0.045 | 0.19× | 0.20× |
| 10,000 | 2.271 | 4.40M | 2.182 | 4.58M | 0.113 | 0.05× | 0.05× |
| 100,000 | 22.050 | 4.54M | 26.224 | 3.81M | 1.002 | 0.05× | 0.04× |
| 1,000,000 | 228.670 | 4.37M | 226.295 | 4.42M | 12.654 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.108 | 0.94× |
| 1 | 5 | 0.310 | 0.461 | 1.49× |
| 1 | 10 | 0.505 | 0.850 | 1.68× |
| 10 | 1 | 0.052 | 0.080 | 1.54× |
| 10 | 5 | 0.243 | 0.420 | 1.73× |
| 10 | 10 | 0.508 | 0.883 | 1.74× |
| 100 | 1 | 0.073 | 0.116 | 1.59× |
| 100 | 5 | 0.255 | 0.598 | 2.34× |
| 100 | 10 | 0.517 | 1.135 | 2.20× |
| 1,000 | 1 | 0.274 | 0.122 | 0.44× |
| 1,000 | 5 | 0.461 | 0.718 | 1.56× |
| 1,000 | 10 | 0.741 | 1.451 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
