# Rising benchmark (`period-over-period rising` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 525.96M | 0.001 | 961.84M | 0.031 | 16.10× | 29.45× |
| 10,000 | 0.007 | 1.45G | 0.004 | 2.30G | 0.037 | 5.43× | 8.60× |
| 100,000 | 0.079 | 1.27G | 0.059 | 1.68G | 0.127 | 1.62× | 2.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.109 | 1.25× |
| 1 | 5 | 0.245 | 0.500 | 2.04× |
| 1 | 10 | 0.394 | 0.910 | 2.31× |
| 10 | 1 | 0.042 | 0.091 | 2.16× |
| 10 | 5 | 0.177 | 0.450 | 2.54× |
| 10 | 10 | 0.387 | 0.943 | 2.44× |
| 100 | 1 | 0.042 | 0.086 | 2.06× |
| 100 | 5 | 0.188 | 0.435 | 2.31× |
| 100 | 10 | 0.371 | 0.946 | 2.55× |
| 1,000 | 1 | 0.045 | 0.104 | 2.31× |
| 1,000 | 5 | 0.192 | 0.488 | 2.53× |
| 1,000 | 10 | 0.383 | 1.029 | 2.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
