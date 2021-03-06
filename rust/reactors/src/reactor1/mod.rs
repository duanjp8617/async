use std::time::{Instant, Duration};
use std::task::{Waker, Context, Poll};
use std::collections::{VecDeque, BTreeMap, BinaryHeap};
use std::cell::{RefCell, Cell};
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use std::cmp::{PartialEq, Eq, Ord, PartialOrd, Ordering, Reverse};
use futures_task::{ArcWake, FutureObj};

// A simple reactor that only supports timeout.

// This single-threaded reactor is motivated by the design of fahrenheit.
// The reactor contains 5 core data structures for storing and manipulating 
// async-tasks.
// start_time : The instant at which the thread-local reactor instance is created.
// timer_heap : A min-heap for storing different timers.
// run_queue : A queue for storing tasks that are about to be waken up.
// task_map : A tree-map for maintaining tasks alive.
// id_counter : A counter that is used to generate unique IDs for tasks.
struct Reactor {
    start_time : Instant,
    timer_heap : RefCell<BinaryHeap<Reverse<Timer>>>,
    run_queue : RefCell<VecDeque<NeedRun>>,
    task_map : RefCell<BTreeMap<usize, Task>>,
    id_counter : Cell<usize>,
}

impl Reactor {
    // Create a new reactor. In this module, the created reactor will be 
    // stored in a thread-local storage and accessed only through an 
    // imutable reference
    fn new() -> Self {
        Self {
            start_time : Instant::now(),
            timer_heap : RefCell::new(BinaryHeap::default()),
            run_queue : RefCell::new(VecDeque::default()),
            task_map : RefCell::new(BTreeMap::default()),
            id_counter : Cell::new(1),
        }
    }

    // Spawn a new task based on a new Future trait object.
    // Currently, the Future trait object must have 'static lifetime and 
    // supports send.
    fn do_spawn<F : Future<Output = ()> + 'static + Send>(&self, f : F) {
        // Generate an unique ID for the new task.
        let task_id = self.next_task_id();
        // Create a new waker based on the generated ID. 
        // In the current implementation, the waker is linked to a particular task 
        // through the use of the ID number. This does incur additional overhead for 
        // retriving and inserting the task object from the task_map, consider 
        // improving this in the future.
        let waker = futures_task::waker(Arc::new(WakerImpl{task_id : task_id}));
        // Construct the task object associated with the async-operation.
        // The actual async-task used in this implementation contains a future object
        // that is stored on the heap. This makes the task freely movable.
        let mut task = Task{task : FutureObj::new(Box::new(f))};

        // Poll the task and get the polling result.
        let res = task.poll(waker);
        match res {
            Poll::Pending => {
                // If the task returns Pending, then the async task falls 
                // into asleep. The waker is added into the timer heap, and 
                // will wake this task up when the timer expires. We must 
                // store the task into the task_map to keep task alive while
                // waiting for the timer to expire.
                self.task_map.borrow_mut().insert(task_id, task);
            }
            _ => {
                // If the task returns Ready(()), then the task finishes,
                // we don't have to track the task and can directly drop the task.
                println!("the task finishes");
            },
            
        }
    }

    // Acquire the next unique ID for a new task.
    fn next_task_id(&self) -> usize {
        let current_id = self.id_counter.get();
        self.id_counter.set(current_id + 1);
        current_id
    }

    // The actual event loop that keeps everything running.
    fn run<F : Future<Output = ()> + 'static + Send>(&self, f:F) {
        // Spawn the initial task.
        self.do_spawn(f);

        loop {
            // Obtain the current eventloop running time.
            let event_loop_tick = Instant::now();
            // Calculate the duration from the current time to 
            // the start of the event loop.
            let expire = &(event_loop_tick - self.start_time);

            // The event loop is separated into the following two parts.

            // 1: The first part is the so-called reactor, which
            // checks whether certain event happens and if so,
            // wakes up the task associated with the event.
            // In this implementation, the only event that may happen
            // is timeout, which is generated by checking the timers stored
            // in the timer_heap. 

            // Check the first element of the timer_heap, which 
            // stores the timer that is the first to expire.
            // If that timer is not expired, we will sleep 
            // until it expires.
            let mut timer_heap = self.timer_heap.borrow_mut();
            timer_heap.peek().map(|next_timer| {
                if *expire < next_timer.0.wakeup_duration {
                    std::thread::sleep(next_timer.0.wakeup_duration - *expire);
                }
            });
            // Iterate through all the expired timers.
            while timer_heap.peek().map_or(false, |next_timer| {
                *expire > next_timer.0.wakeup_duration
            }) {
                // Wake up the task associated with the expired timer 
                // by calling wake_by_ref. The waker contains the task 
                // ID associated with this timer. The wake_by_ref call
                // will add the task ID into the run_queue.
                let timer = timer_heap.pop().unwrap();
                timer.0.waker.wake_by_ref();
            }
            // Destroy the RefMut when we finish proessing the timer_heap.
            // Note: this is compulsory, as the executor part of the reactor
            // may add new timers into the timer_heap in case that the tasks
            // need to sleep for a certain amount of time. If we keep this 
            // RefMut alive, we will panic the program.
            drop(timer_heap);

            // 2. The second part is the executor. In this part, the reactor
            // will schedule every resumable tasks stored in the run_queue
            // to run again. 

            // Iterate through the run_queue.
            // Each item of the run_queue contains an ID which is 
            // linked to a task that should be resumed.  
            let mut run_queue = self.run_queue.borrow_mut();
            for _ in 0..run_queue.len() {
                // Remove the task from the task_map
                let needrun = run_queue.pop_front().unwrap();
                let mut task = self.task_map.borrow_mut().remove(&needrun.task_id).unwrap();
                // Resume the task by polling.
                // Note that a task may do the following two things when being executed:
                // 1. Append a new timer to the timer_heap and wait for some time
                // 2. Spawn a new async task and insert the task inside the task_map.
                // Whether using a single RefMut to access the core data structures 
                // depends on the following analysis:
                // 1. The run_queue of reactor is not touched by task.poll, so holding a 
                // RefMut to the run_queue is fine.
                // 2. The timer_heap may be modified by task.poll, so holding a RefMut 
                // to timer_heap when executing task.poll will panic the program. 
                // Since we drop the RefMut to timer_heap  at the end of the reactor part, 
                // this works fine too.
                // 3. The task_map may be modified by task.poll, so we can't hold a RefMut
                // to the task_map when executing task.poll. This is why we create temporary
                // RefMut to task_map before and after the call to task.poll
                match task.poll(needrun.waker) {
                    Poll::Pending => {
                        // If the task blocks again, insert the task back into the 
                        // task_map and wait for later wakeup.
                        self.task_map.borrow_mut().insert(needrun.task_id, task);
                    },
                    _ => {
                        // If the task finishes, just continue and drops the finished
                        // task.
                        println!("task finishes");
                    }
                }
            }

            // Check whether there are pending tasks. 
            // If not, stops the eventloop.
            if self.task_map.borrow().len() == 0 {
                break;
            }
        }
    }
}

// The reactor is stored inside a thread local storage and read-only.
thread_local! {
    static REACTOR : Reactor = Reactor::new()
}

// The implementation of the waker, which only contains a task_id.
struct WakerImpl {
    task_id : usize,
}

// Implementing the ArcWake trait for the WakerImpl.
impl ArcWake for WakerImpl {
    // When a timer is popped off from the timer_heap, it needs to
    // wakeup the corresponding task. It does so by constructing 
    // a new NeedRun and inserting it into the run_queue.
    fn wake_by_ref(arc_self : &Arc<Self>) {
        // Construct a new NeedRun.
        let next_need_run = NeedRun {
            task_id : (**arc_self).task_id,
            waker : futures_task::waker(arc_self.clone()),
        };
        // Insert the NeedRun into the run queue. 
        // This is called in the reactor part, and acquring 
        // a RefMut to the run_queue is safe.
        REACTOR.with(|reactor|{
            reactor.run_queue.borrow_mut().push_back(next_need_run);
        });
    }
}

// An item stored in the run_queue, indicating which task should 
// be woken up and resumed.
// task_id : the ID of the task which needs to be resumed.
// waker : the waker passed in as the context when calling poll.
struct NeedRun {
    task_id : usize,
    waker : Waker,
}

// The actual representation of an asynchronous task in this implementation.
// task : A box pointing to an heap-allocated area for storing the future state machine.
struct Task {
    task : FutureObj<'static, ()>,
}

impl Task {
    // A wrapper to poll the contained FutureObj. 
    // This wrapper prepares the arguments for calling
    // Future::Poll
    fn poll(&mut self, waker : Waker) -> Poll<()> {
        let pinned = Pin::new(&mut self.task);
        let mut ctx = Context::from_waker(&waker);
        Future::poll(pinned, &mut ctx)    
    }
}

// Implementation of the PartialEq trait for Timer
impl PartialEq<Timer> for Timer {
    fn eq(&self, other: &Timer) -> bool {
        self.wakeup_duration == other.wakeup_duration
    }
}

// Implementation of the Eq trait for Timer
impl Eq for Timer {}

// Implementation of the PartialOrder trait for Timer
impl PartialOrd<Timer> for Timer {
    fn partial_cmp(&self, other: &Timer) -> Option<Ordering> {
        if self.wakeup_duration == other.wakeup_duration {
            Some(Ordering::Equal)
        }
        else if self.wakeup_duration > other.wakeup_duration {
            Some(Ordering::Greater)
        }
        else {
            Some(Ordering::Less)
        }
    }
}

// Implementation of the Order trait for Timer
impl Ord for Timer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// The item stored in the timer_heap.
// wakeup_duration : The duration from the start time of the reactor to the 
// time that this timer expires. If wakeup_duration is shorter than the duration
// from the start time to the current instant, then the timer expires and should 
// wake up the corresponding task.
// waker : When the timer expires, the event loop calls waker.wake_by_ref
// to insert the NeedRun into run_queue.
struct Timer {
    wakeup_duration : Duration,
    waker : Waker,
}

// A future object that sleeps for a certain amount of time 
// as indicated in the contained duration and resumes the execution
// after sleep.
struct Timeout {
    duration : Duration,
}

// Timeout is Unpin, as we need to change the 
// content of the duration to indicate whether 
// the task has resumed from the sleep.
impl Unpin for Timeout {}

impl Timeout {
    fn new(duration : Duration) -> Self {
        Timeout {
            duration : duration,
        }
    }
}

impl Future for Timeout {
    type Output = ();

    // Poll the Timeout object. Depending on the content of the contained duration,
    // the task will do the following two things:
    // 1. If the contained duration is 0, then either the task has resumed after sleep,
    // or the task does not need to sleep at all. A Poll::Ready(()) is returned to
    // resume the execution of the task.
    // 2. If the contained duration is not zero, then we insert a Timer into the timer_heap,
    // and returns Poll::Pending to suspend the execution of the task.
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.duration == Duration::new(0, 0) {
            Poll::Ready(())
        }
        else {
            // Prepare the variables.
            let duration = self.duration.clone();
            self.get_mut().duration = Duration::new(0, 0);
            let waker = ctx.waker().clone();
            
            REACTOR.with(|reactor|{
                // Construct a timer.
                let timer = Timer {
                    wakeup_duration : (Instant::now() - reactor.start_time) + duration,
                    waker : waker
                };
                // Insert the timer into the reactor.
                reactor.timer_heap.borrow_mut().push(Reverse(timer));
            });

            Poll::Pending
        }
    }
}

// The entry point of the async eventloop.
fn run<F : Future<Output = ()> + 'static + Send>(f : F) {
    REACTOR.with(|reactor| {
        reactor.run(f);
    });
}

// Spawning a new Future task inside the eventloop.
fn spawn<F : Future<Output = ()> + 'static + Send>(f : F) {
    REACTOR.with(|reactor| {
        reactor.do_spawn(f);
    });
}

async fn sleep_sub_task(id : i32) {
    println!("sleep sub-task {} is created", id);
    Timeout::new(Duration::from_secs(10)).await;
    println!("sleep sub-task {} finishes", id);
}

async fn sleep_task() {
    for i in 1..11 {
        println!("main task sleep for 1s");
        Timeout::new(Duration::from_secs(1)).await;
        println!("create sleep sub task {}", i);
        spawn(sleep_sub_task(i));
    }
}

pub fn launch() {
    run(sleep_task());
}