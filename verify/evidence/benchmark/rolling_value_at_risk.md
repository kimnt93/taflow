# RollingValueAtRisk benchmark (`ValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.129 | 7.75M | 0.136 | 7.33M | 0.321 | 2.49× | 2.35× |
| 10,000 | 1.280 | 7.81M | 1.278 | 7.82M | 1.835 | 1.43× | 1.44× |
| 100,000 | 13.184 | 7.58M | 13.310 | 7.51M | 15.771 | 1.20× | 1.18× |
| 1,000,000 | 127.682 | 7.83M | 129.189 | 7.74M | 163.673 | 1.28× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.485 | 3.54× |
| 1 | 5 | 0.329 | 1.236 | 3.76× |
| 1 | 10 | 0.490 | 2.616 | 5.34× |
| 10 | 1 | 0.051 | 0.232 | 4.57× |
| 10 | 5 | 0.231 | 1.149 | 4.97× |
| 10 | 10 | 0.462 | 2.461 | 5.33× |
| 100 | 1 | 0.063 | 0.258 | 4.08× |
| 100 | 5 | 0.250 | 1.425 | 5.69× |
| 100 | 10 | 0.511 | 2.636 | 5.16× |
| 1,000 | 1 | 0.191 | 0.419 | 2.20× |
| 1,000 | 5 | 0.331 | 2.485 | 7.51× |
| 1,000 | 10 | 0.910 | 4.989 | 5.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
