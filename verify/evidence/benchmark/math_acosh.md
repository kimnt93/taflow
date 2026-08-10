# MathAcosh benchmark (`numpy.arccosh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.99M | 0.012 | 83.90M | 0.021 | 1.60× | 1.74× |
| 10,000 | 0.103 | 96.85M | 0.098 | 102.39M | 0.108 | 1.05× | 1.11× |
| 100,000 | 1.010 | 99.00M | 0.970 | 103.08M | 0.990 | 0.98× | 1.02× |
| 1,000,000 | 10.336 | 96.75M | 9.859 | 101.43M | 9.843 | 0.95× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.102 | 0.90× |
| 1 | 5 | 0.331 | 0.319 | 0.96× |
| 1 | 10 | 0.480 | 0.703 | 1.47× |
| 10 | 1 | 0.045 | 0.060 | 1.34× |
| 10 | 5 | 0.202 | 0.301 | 1.49× |
| 10 | 10 | 0.501 | 0.607 | 1.21× |
| 100 | 1 | 0.054 | 0.058 | 1.08× |
| 100 | 5 | 0.235 | 0.317 | 1.35× |
| 100 | 10 | 0.518 | 0.687 | 1.33× |
| 1,000 | 1 | 0.061 | 0.065 | 1.06× |
| 1,000 | 5 | 0.264 | 0.397 | 1.51× |
| 1,000 | 10 | 0.512 | 0.750 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
