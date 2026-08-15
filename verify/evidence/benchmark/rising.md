# Rising benchmark (`period-over-period rising` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.27M | 0.006 | 172.34M | 0.028 | 4.42× | 4.91× |
| 10,000 | 0.048 | 208.55M | 0.045 | 223.21M | 0.036 | 0.74× | 0.80× |
| 100,000 | 0.446 | 224.14M | 0.424 | 235.88M | 0.113 | 0.25× | 0.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.144 | 1.98× |
| 1 | 5 | 0.317 | 0.498 | 1.57× |
| 1 | 10 | 0.394 | 1.016 | 2.58× |
| 10 | 1 | 0.048 | 0.092 | 1.93× |
| 10 | 5 | 0.184 | 0.441 | 2.39× |
| 10 | 10 | 0.373 | 0.914 | 2.45× |
| 100 | 1 | 0.045 | 0.086 | 1.91× |
| 100 | 5 | 0.183 | 0.446 | 2.44× |
| 100 | 10 | 0.401 | 0.904 | 2.26× |
| 1,000 | 1 | 0.046 | 0.091 | 1.98× |
| 1,000 | 5 | 0.205 | 0.496 | 2.42× |
| 1,000 | 10 | 0.424 | 1.060 | 2.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
