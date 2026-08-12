# SpreadZScore benchmark (`rolling hedged-spread z-score` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.092 | 10.93M | 0.091 | 10.94M | 0.411 | 4.49× | 4.50× |
| 10,000 | 0.876 | 11.41M | 0.901 | 11.10M | 2.876 | 3.28× | 3.19× |
| 100,000 | 8.927 | 11.20M | 9.115 | 10.97M | 31.565 | 3.54× | 3.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.201 | 2.19× |
| 1 | 5 | 0.338 | 0.763 | 2.26× |
| 1 | 10 | 0.466 | 1.529 | 3.28× |
| 10 | 1 | 0.054 | 0.149 | 2.79× |
| 10 | 5 | 0.229 | 0.748 | 3.27× |
| 10 | 10 | 0.475 | 1.520 | 3.20× |
| 100 | 1 | 0.059 | 0.253 | 4.30× |
| 100 | 5 | 0.243 | 1.451 | 5.98× |
| 100 | 10 | 0.519 | 3.383 | 6.52× |
| 1,000 | 1 | 0.150 | 0.520 | 3.45× |
| 1,000 | 5 | 0.316 | 2.032 | 6.43× |
| 1,000 | 10 | 0.590 | 3.928 | 6.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
