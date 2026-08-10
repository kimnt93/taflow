# McClellanOscillator benchmark (`McClellanOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.08M | 0.008 | 129.61M | 8.537 | 931.23× | 1106.52× |
| 10,000 | 0.052 | 192.69M | 0.049 | 206.11M | 83.812 | 1614.95× | 1727.48× |
| 100,000 | 0.512 | 195.28M | 0.433 | 230.89M | 839.069 | 1638.52× | 1937.30× |
| 1,000,000 | 5.489 | 182.20M | 4.668 | 214.24M | 8400.765 | 1530.59× | 1799.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.211 | 0.284 | 1.35× |
| 1 | 5 | 0.359 | 1.024 | 2.85× |
| 1 | 10 | 0.474 | 2.073 | 4.37× |
| 10 | 1 | 0.048 | 0.288 | 6.01× |
| 10 | 5 | 0.223 | 1.677 | 7.54× |
| 10 | 10 | 0.470 | 2.929 | 6.23× |
| 100 | 1 | 0.050 | 1.066 | 21.44× |
| 100 | 5 | 0.245 | 5.609 | 22.89× |
| 100 | 10 | 0.527 | 10.998 | 20.89× |
| 1,000 | 1 | 0.062 | 8.518 | 138.42× |
| 1,000 | 5 | 0.272 | 47.804 | 175.46× |
| 1,000 | 10 | 0.692 | 102.397 | 148.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
