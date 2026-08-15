# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.15M | 0.009 | 117.53M | 0.035 | 3.49× | 4.10× |
| 10,000 | 0.082 | 122.37M | 0.079 | 126.84M | 0.105 | 1.28× | 1.33× |
| 100,000 | 0.853 | 117.29M | 0.810 | 123.39M | 0.811 | 0.95× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.109 | 1.53× |
| 1 | 5 | 0.201 | 0.419 | 2.08× |
| 1 | 10 | 0.385 | 0.852 | 2.21× |
| 10 | 1 | 0.040 | 0.084 | 2.10× |
| 10 | 5 | 0.228 | 0.437 | 1.92× |
| 10 | 10 | 0.400 | 0.870 | 2.18× |
| 100 | 1 | 0.047 | 0.090 | 1.92× |
| 100 | 5 | 0.178 | 0.436 | 2.45× |
| 100 | 10 | 0.433 | 0.952 | 2.20× |
| 1,000 | 1 | 0.049 | 0.098 | 2.00× |
| 1,000 | 5 | 0.189 | 0.451 | 2.39× |
| 1,000 | 10 | 0.460 | 1.044 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
