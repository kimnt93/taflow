# RollingCalmar benchmark (`rolling calmar on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.95M | 0.042 | 23.68M | 0.243 | 5.57× | 5.75× |
| 10,000 | 0.419 | 23.84M | 0.423 | 23.66M | 1.458 | 3.48× | 3.45× |
| 100,000 | 4.748 | 21.06M | 4.499 | 22.23M | 19.766 | 4.16× | 4.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.119 | 1.46× |
| 1 | 5 | 0.213 | 0.499 | 2.34× |
| 1 | 10 | 0.469 | 1.176 | 2.50× |
| 10 | 1 | 0.054 | 0.096 | 1.76× |
| 10 | 5 | 0.231 | 0.453 | 1.96× |
| 10 | 10 | 0.419 | 0.898 | 2.14× |
| 100 | 1 | 0.053 | 0.210 | 3.98× |
| 100 | 5 | 0.462 | 1.147 | 2.48× |
| 100 | 10 | 0.450 | 2.102 | 4.67× |
| 1,000 | 1 | 0.097 | 0.352 | 3.63× |
| 1,000 | 5 | 0.242 | 1.315 | 5.42× |
| 1,000 | 10 | 0.445 | 2.629 | 5.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
