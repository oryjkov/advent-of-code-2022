Things that were interesting:

 - group_by that needed external state

   Commands are easily identifiable as they start with a `$` sign. Command output follows its command.
   It was useful to group each command together with its output. I could not think of a way to use
   group_by without external state to accomplish this task. With external state I would toggle the state
   on every new command and keep it that way until the next command was seen.

 - Great exercise on Rc, RefCell to make the Dir structure work.

   This was the first time building a recursive data structure. I needed a way to refer to any node in
   the tree and be able to update the tree via that reference (`RefCell`). Having more than one reference
   to a subtree meant using `Rc`.

 - Exercise in flattening a tree.

   I needed a way to find the size of each directory in the tree and iterate over that list. Traversing the
   tree recursively with each call outputting the list of directories found and their sizes, with the root
   always being at head. In order to merge the results, current directory's size is increased by the size of 
   its immediate children and appended with the remaining elements of the child's output.
   
