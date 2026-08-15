# OpeningRange benchmark (`anchored opening range` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.99M | 0.007 | 137.21M | 0.546 | 52.93× | 74.87× |
| 10,000 | 0.067 | 150.21M | 0.058 | 171.26M | 5.343 | 80.26× | 91.50× |
| 100,000 | 0.585 | 171.00M | 0.541 | 184.82M | 53.029 | 90.68× | 98.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.121 | 1.69× |
| 1 | 5 | 0.202 | 0.433 | 2.15× |
| 1 | 10 | 0.394 | 0.962 | 2.44× |
| 10 | 1 | 0.051 | 0.102 | 2.00× |
| 10 | 5 | 0.183 | 0.457 | 2.49× |
| 10 | 10 | 0.393 | 0.981 | 2.50× |
| 100 | 1 | 0.043 | 0.143 | 3.30× |
| 100 | 5 | 0.211 | 0.778 | 3.69× |
| 100 | 10 | 0.456 | 1.460 | 3.20× |
| 1,000 | 1 | 0.061 | 0.633 | 10.32× |
| 1,000 | 5 | 0.243 | 3.144 | 12.93× |
| 1,000 | 10 | 0.478 | 6.414 | 13.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
