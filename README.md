# 🪵 Log - Index log files

🪵 Log - or wood log - is a persistent keyd data structure that allows fast storage of data and retrieval by key.

## CLI

## How it works

- Set key
- Append only
- Retrieve by key
- Update data by adding with the same key

## When to use

- Write once read often
- Need something simple
- No infrastructure needed (other than file system)

## When not to use

- Modify data
- Upstart cost can be high
- SQLite could solve your problems - you might want the features later
