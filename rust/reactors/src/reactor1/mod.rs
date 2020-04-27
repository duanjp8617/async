use std::time::{Instant, Duration};
use std::task::{Waker, Context, Poll};
use std::collections::{VecDeque, BTreeMap, BinaryHeap};
use std::cell::{RefCell, Cell};
use futures_task::{ArcWake, FutureObj};
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use std::cmp::{PartialEq, Eq, Ord, PartialOrd, Ordering, Reverse};

// Reactor1:
// A single-thread reactor motivated by fahrenheit, that only supports 
// async sleep. 

// This reactor contains four core data structures for storing and manipulating async tasks.
// 1. The startup time of the reactor.

// 2. A queue that stores timers. Each timer contains a duration
// in milliseconds and a waker. When the elapsed time against the 
// startup time is larger than the duration stored in the timer,
// the timer will be popped off from the queue, and the waker will be 
// called to insert the waker into the run queue.

// 3. A run queue that stores a Wakeup struct. The Wakeup struct contains
// a task ID and a waker. The reactor will pop off all the the Wakeup stored
// in the run queue in each iteration. Each Wakeup stores a task ID and a 
// waker. The task ID is used to retrieve the task from the HashMap. The 
// waker is used to poll the retrived task.

// 4. A HashMap for storing the tasks and keep them alive. Whenever the task
// is deleted from the HashMap, the task's lifetime ends and the task
// will be dropped.

// 5. A increase-only counter for generating unique task IDs

// Reactor:
// The core reactor structure.
// born_time : 
struct Reactor {
    start_time : Instant,
    timer_queue : RefCell<BinaryHeap<Reverse<Timer>>>,
    run_queue : RefCell<VecDeque<NeedRun>>,
    task_map : RefCell<BTreeMap<usize, Task>>,
    id_counter : Cell<usize>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            start_time : Instant::now(),
            timer_queue : RefCell::new(BinaryHeap::default()),
            run_queue : RefCell::new(VecDeque::default()),
            task_map : RefCell::new(BTreeMap::default()),
            id_counter : Cell::new(1),
        }
    }

    fn do_spawn<F : Future<Output = ()> + 'static + Send>(&self, f : F) {
        let task_id = self.next_task_id();
        let waker = futures_task::waker(Arc::new(WakerImpl{task_id : task_id}));
        let mut task = Task{task : FutureObj::new(Box::new(f))};

        let res = task.poll(waker);

        match res {
            Poll::Ready(_) => {
                println!("the task finishes");
            },
            Poll::Pending => {
                self.task_map.borrow_mut().insert(task_id, task);
            }
        }
    }

    fn next_task_id(&self) -> usize {
        let current_id = self.id_counter.get();
        self.id_counter.set(current_id + 1);
        current_id
    }

    fn run<F : Future<Output = ()> + 'static + Send>(&self, f:F) {
        println!("sth");
        self.do_spawn(f);

        loop {
            let event_loop_tick = Instant::now();
            let expire = &(event_loop_tick - self.start_time);

            // sleep some time
            let mut timer_queue = self.timer_queue.borrow_mut();
            timer_queue.peek().map(|next_timer| {
                if *expire < next_timer.0.wakeup_duration {
                    std::thread::sleep(next_timer.0.wakeup_duration - *expire);
                }
            });

            // find out the total number of expired timers
            while timer_queue.peek().map_or(false, |next_timer| {
                *expire > next_timer.0.wakeup_duration
            }) {
                let timer = timer_queue.pop().unwrap();
                timer.0.waker.wake_by_ref();
            }
            drop(timer_queue);

            let mut run_queue = self.run_queue.borrow_mut();
            for _ in 0..run_queue.len() {
                let needrun = run_queue.pop_front().unwrap();
                let mut task = self.task_map.borrow_mut().remove(&needrun.task_id).unwrap();
                
                match task.poll(needrun.waker) {
                    Poll::Pending => {
                        self.task_map.borrow_mut().insert(needrun.task_id, task);
                    },
                    _ => {
                        println!("task finishes");
                    }
                }
            }

            if self.task_map.borrow().len() == 0 {
                break;
            }
        }
    }
}

thread_local! {
    static REACTOR : Reactor = Reactor::new()
}

struct WakerImpl {
    task_id : usize,
}

impl ArcWake for WakerImpl {
    fn wake_by_ref(arc_self : &Arc<Self>) {
        let next_need_run = NeedRun {
            task_id : (**arc_self).task_id,
            waker : futures_task::waker(arc_self.clone()),
        };

        REACTOR.with(|reactor|{
            reactor.run_queue.borrow_mut().push_back(next_need_run);
        });
    }
}

struct NeedRun {
    task_id : usize,
    waker : Waker,
}

struct Task {
    task : FutureObj<'static, ()>,
}

impl Task {
    // A wrapper for polling. The only purpose of this 
    // function is to prepare inputs for Future::poll
    fn poll(&mut self, waker : Waker) -> Poll<()> {
        let pinned = Pin::new(&mut self.task);
        let mut ctx = Context::from_waker(&waker);
        Future::poll(pinned, &mut ctx)    
    }
}

impl PartialEq<Timer> for Timer {
    fn eq(&self, other: &Timer) -> bool {
        self.wakeup_duration == other.wakeup_duration
    }
}

impl Eq for Timer {}

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

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Timer {
    wakeup_duration : Duration,
    waker : Waker,
}

struct Timeout {
    duration : Duration,
}

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

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.duration == Duration::new(0, 0) {
            Poll::Ready(())
        }
        else {
            let duration = self.duration.clone();
            self.get_mut().duration = Duration::new(0, 0);
            let waker = ctx.waker().clone();
            
            REACTOR.with(|reactor|{
                let timer = Timer {
                    wakeup_duration : (Instant::now() - reactor.start_time) + duration,
                    waker : waker
                };
                reactor.timer_queue.borrow_mut().push(Reverse(timer));
            });

            Poll::Pending
        }
    }
}

fn run<F : Future<Output = ()> + 'static + Send>(f : F) {
    REACTOR.with(|reactor| {
        reactor.run(f);
    });
}

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