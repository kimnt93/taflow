# MedianChannel benchmark (`MedianChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.163 | 6.15M | 0.168 | 5.95M | 0.939 | 5.77× | 5.59× |
| 10,000 | 1.632 | 6.13M | 1.660 | 6.02M | 7.585 | 4.65× | 4.57× |
| 100,000 | 16.735 | 5.98M | 16.494 | 6.06M | 77.119 | 4.61× | 4.68× |
| 1,000,000 | 165.098 | 6.06M | 166.468 | 6.01M | 830.815 | 5.03× | 4.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.373 | 3.74× |
| 1 | 5 | 0.288 | 1.386 | 4.81× |
| 1 | 10 | 0.513 | 2.641 | 5.15× |
| 10 | 1 | 0.051 | 0.249 | 4.88× |
| 10 | 5 | 0.235 | 1.412 | 6.02× |
| 10 | 10 | 0.484 | 2.897 | 5.99× |
| 100 | 1 | 0.069 | 0.326 | 4.74× |
| 100 | 5 | 0.260 | 1.883 | 7.23× |
| 100 | 10 | 0.565 | 3.438 | 6.09× |
| 1,000 | 1 | 0.218 | 1.192 | 5.48× |
| 1,000 | 5 | 0.383 | 5.487 | 14.33× |
| 1,000 | 10 | 0.712 | 11.059 | 15.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
