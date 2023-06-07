# Difficulty Pallet

The difficulty pallet is responsible for storing the difficulty of the current block, and calculating the difficulty for the next block.
The fact that this logic lives in a pallet means that we canchange our difficulty adjustment algorithm with a runtime upgrade, which is cool.
