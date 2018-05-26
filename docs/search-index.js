var searchIndex = {};
searchIndex["lockfree"] = {"doc":"","items":[[0,"prelude","lockfree","Provides convenient re-exports.",null,null],[0,"queue","","Provides a queue without FIFO garantees on multithread environments, but still concurrent and lock-free. Single thread environments still have FIFO garanteees.",null,null],[3,"Queue","lockfree::queue","A lock-free concurrent queue, but without FIFO garantees on multithreaded environments. Single thread environments still have FIFO garantees. The queue is based on subqueues which threads try to take, modify and then publish. If necessary, subqueues are joint.",null,null],[3,"Inspector","","An iterator which inspects a subqueue.",null,null],[11,"new","","Creates a new empty queue.",0,{"o":{"n":"self"}}],[11,"push","","Pushes a value in the back of the queue.",0,{"i":[{"n":"self"},{"n":"t"}]}],[11,"pop","","Pops a value from the front of the queue.",0,{"i":[{"n":"self"}],"o":{"n":"option"}}],[11,"inspect","","Creates an inspector on the current subqueue.",0,{"i":[{"n":"self"}],"o":{"n":"inspector"}}],[11,"drop","","",0,{"i":[{"n":"self"}]}],[11,"fmt","","",0,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"next","","",1,{"i":[{"n":"self"}],"o":{"n":"option"}}],[11,"drop","","",1,{"i":[{"n":"self"}]}],[11,"fmt","","",1,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}]],"paths":[[3,"Queue"],[3,"Inspector"]]};
initSearch(searchIndex);
