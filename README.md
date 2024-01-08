Solved parts 1 & 2

This was made easier by the fact that you never had to actually draw a path, all galaxies can be found just by taking the difference in their coordinates. Part 2 avoids having to build a matrix that's too large to represent in memory just by tracking offsets. BTreeSet was chosen because it has the ordering properties of a Vec but has quick lookups like a hashmap.