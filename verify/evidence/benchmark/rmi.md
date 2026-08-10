# RelativeMomentumIndex benchmark

Oracle: Wickra `RMI` | Correctness: MATCH

| Bars | TAFlow ms | Oracle ms | Speedup |
|---:|---:|---:|---:|
| 1,000 | 0.0100 | 0.0674 | 6.75× |
| 10,000 | 0.0715 | 0.5673 | 7.94× |
| 100,000 | 0.7060 | 4.5590 | 6.46× |
| 1,000,000 | 7.4867 | 46.6721 | 6.23× |

## Warmed continuation

| Chunk | Bars/s |
|---:|---:|
| 1 | 4,081,044 |
| 10 | 9,912,178 |
| 1,000 | 69,386,622 |
